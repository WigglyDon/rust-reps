//! Portable workshop editor and information-pane state.
use crate::{
    client::{CursorPosition, EditorAction, EditorBuffer},
    curriculum::workshop::{STEPS, TRUST},
    workshop::{Activity, SOURCE_LIMIT, Session},
    workshop_process::safe_text,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pane {
    Guidance,
    Compiler,
    Program,
}

#[derive(Clone, Debug)]
pub enum Action {
    Edit(EditorAction),
    Cursor(CursorPosition),
    ToggleFocus,
    CyclePane,
    PreviousStep,
    NextStep,
    Scroll(i32),
    Save,
    Compile,
    Run,
    Stop,
    Quit,
    Ignore,
}

pub struct WorkshopClient {
    pub session: Session,
    pub editor: EditorBuffer,
    pub information_focus: bool,
    pub pane: Pane,
    pub step: usize,
    pub scroll: [usize; 3],
    pub quit: bool,
}

impl WorkshopClient {
    pub fn new(session: Session) -> Self {
        let mut editor = EditorBuffer::new();
        editor.set_text(&session.source);
        editor.set_cursor(CursorPosition::origin());
        Self {
            session,
            editor,
            information_focus: false,
            pane: Pane::Guidance,
            step: 0,
            scroll: [0; 3],
            quit: false,
        }
    }

    pub fn pane_index(&self) -> usize {
        match self.pane {
            Pane::Guidance => 0,
            Pane::Compiler => 1,
            Pane::Program => 2,
        }
    }

    pub fn handle(&mut self, action: Action) {
        match action {
            Action::ToggleFocus => self.information_focus = !self.information_focus,
            Action::CyclePane => {
                self.pane = match self.pane {
                    Pane::Guidance => Pane::Compiler,
                    Pane::Compiler => Pane::Program,
                    Pane::Program => Pane::Guidance,
                }
            }
            Action::PreviousStep => {
                self.step = self.step.saturating_sub(1);
                self.pane = Pane::Guidance;
                self.scroll[0] = 0;
            }
            Action::NextStep => {
                self.step = (self.step + 1).min(STEPS.len() - 1);
                self.pane = Pane::Guidance;
                self.scroll[0] = 0;
            }
            Action::Scroll(amount) => {
                let index = self.pane_index();
                self.scroll[index] = self.scroll[index].saturating_add_signed(amount as isize);
            }
            Action::Edit(edit) if !self.information_focus => {
                let before = self.editor.clone();
                self.editor.apply_action(edit);
                let text = self.editor.text();
                if text.len() > SOURCE_LIMIT {
                    self.editor = before;
                    self.session.message =
                        "Source limit: 256 KiB. Edit rejected; buffer preserved.".into();
                } else {
                    self.session.change_source(text);
                }
            }
            Action::Cursor(cursor) => {
                self.information_focus = false;
                self.editor.set_cursor(cursor);
            }
            Action::Save => {
                self.session.save();
            }
            Action::Compile => {
                self.session.compile();
                self.pane = Pane::Compiler;
                self.scroll[1] = 0;
            }
            Action::Run => {
                self.session.run();
                self.pane = Pane::Program;
                self.scroll[2] = 0;
            }
            Action::Stop => self.session.stop(),
            Action::Quit => {
                // Termination is responsive even when source cannot be saved.
                self.session.stop();
                self.quit = self.session.save();
            }
            _ => {}
        }
    }

    pub fn information(&self) -> String {
        if self.pane == Pane::Guidance {
            let (title, prose) = STEPS[self.step];
            return format!("PREVIEW — subject to Don's review\n{title}\n\n{prose}\n\n{TRUST}");
        }
        let activity = if self.pane == Pane::Compiler {
            Activity::Compile
        } else {
            Activity::Run
        };
        let Some(record) = self.session.output(activity) else {
            return format!(
                "{activity:?}: no observed output yet.\n\n{}\n\n{TRUST}",
                self.session
                    .run_block_reason()
                    .unwrap_or("Run is available for this successful build.")
            );
        };
        let identity = if record.revision == self.session.revision
            && record.build_attempt == self.session.build_attempt
        {
            "displayed revision"
        } else {
            "PREVIOUS revision/build attempt; Compile current source before Run"
        };
        let outcome = record.status_text();
        format!(
            "{:?} — revision {}, build attempt {} ({identity})\n{}\n\nStatus: {outcome}\n{}\n\nstdout (observed):\n{}\n\nstderr (observed):\n{}\n\n{TRUST}",
            record.activity,
            record.revision,
            record.build_attempt,
            record.command,
            if record.capture.truncated {
                "OUTPUT TRUNCATED at 64 KiB per stream; job stopped"
            } else {
                ""
            },
            safe_text(&record.capture.stdout),
            safe_text(&record.capture.stderr)
        )
    }
}
