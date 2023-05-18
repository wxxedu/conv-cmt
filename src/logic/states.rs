use crate::ui::ui::UI;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct StateData<'a, T: UI> {
    tmp: &'a mut str,
    ui: &'a mut T,
}

pub trait StateTrait<'a, T: UI> {
    fn next(&'a mut self) -> State<'a, T>;
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum State<'a, T: UI> {
    Entry(Entry<'a, T>),
    Staging(Staging<'a, T>),
    CommitType(CommitType<'a, T>),
    CommitScope(CommitScope<'a, T>),
    CommitSubject(CommitSubject<'a, T>),
    CommitDescription(CommitDescription<'a, T>),
    Confirm(Confirm<'a, T>),
    SemanticVersion(SemanticVersion<'a, T>),
    ChangeLog(ChangeLog<'a, T>),
    Commit(Commit<'a, T>),
    Push(Push<'a, T>),
    Done(Done<'a, T>),
}

impl<'a, T: UI> StateTrait<'a, T> for State<'a, T> {
    fn next(&'a mut self) -> State<'a, T> {
        match self {
            State::Entry(state) => state.next(),
            State::Staging(state) => state.next(),
            State::CommitType(state) => state.next(),
            State::CommitScope(state) => state.next(),
            State::CommitSubject(state) => state.next(),
            State::CommitDescription(state) => state.next(),
            State::Confirm(state) => state.next(),
            State::SemanticVersion(state) => state.next(),
            State::ChangeLog(state) => state.next(),
            State::Commit(state) => state.next(),
            State::Push(state) => state.next(),
            State::Done(state) => state.next(),
        }
    }
}

macro_rules! state {
    ($($name:ident),+) => {
        $(
            #[derive(Debug, PartialEq, Eq, Hash)]
            pub struct $name<'a, T: UI> {
                data: &'a mut StateData<'a, T>,
            }

            impl<'a, T: UI> $name<'a, T> {
                pub fn new(data: &'a mut StateData<'a, T>) -> Self {
                    Self { data }
                }
            }

            impl<'a, T: UI> Into<State<'a, T>> for $name<'a, T> {
                fn into(self) -> State<'a, T> {
                    State::$name(self)
                }
            }
        )+
    };
}

state!(
    Entry,
    Staging,
    CommitType,
    CommitScope,
    CommitSubject,
    CommitDescription,
    Confirm,
    SemanticVersion,
    ChangeLog,
    Commit,
    Push,
    Done
);

impl<'a, T: UI> StateTrait<'a, T> for Entry<'a, T> {
    fn next(&'a mut self) -> State<'a, T> {
        self.data.ui.write("Welcome to gcmt!\n");
        Staging::new(self.data).into()
    }
}

impl<'a, T: UI> StateTrait<'a, T> for Staging<'a, T> {
    fn next(&'a mut self) -> State<'a, T> {
        todo!()
    }
}

impl<'a, T: UI> StateTrait<'a, T> for CommitType<'a, T> {
    fn next(&'a mut self) -> State<'a, T> {
        todo!()
    }
}

impl<'a, T: UI> StateTrait<'a, T> for CommitScope<'a, T> {
    fn next(&'a mut self) -> State<'a, T> {
        todo!()
    }
}

impl<'a, T: UI> StateTrait<'a, T> for CommitSubject<'a, T> {
    fn next(&'a mut self) -> State<'a, T> {
        todo!()
    }
}

impl<'a, T: UI> StateTrait<'a, T> for CommitDescription<'a, T> {
    fn next(&'a mut self) -> State<'a, T> {
        todo!()
    }
}

impl<'a, T: UI> StateTrait<'a, T> for Confirm<'a, T> {
    fn next(&'a mut self) -> State<'a, T> {
        todo!()
    }
}

impl<'a, T: UI> StateTrait<'a, T> for SemanticVersion<'a, T> {
    fn next(&'a mut self) -> State<'a, T> {
        todo!()
    }
}

impl<'a, T: UI> StateTrait<'a, T> for ChangeLog<'a, T> {
    fn next(&'a mut self) -> State<'a, T> {
        todo!()
    }
}

impl<'a, T: UI> StateTrait<'a, T> for Commit<'a, T> {
    fn next(&'a mut self) -> State<'a, T> {
        todo!()
    }
}

impl<'a, T: UI> StateTrait<'a, T> for Push<'a, T> {
    fn next(&'a mut self) -> State<'a, T> {
        todo!()
    }
}

impl<'a, T: UI> StateTrait<'a, T> for Done<'a, T> {
    fn next(&'a mut self) -> State<'a, T> {
        todo!()
    }
}
