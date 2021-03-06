
use {
    anyhow::Result,
    crate::{
        persist::Level,
    },
    super::{
        Context,
        transition::StateTransition,
    },
};


/// One of the screens of Lapin.
///
pub trait State {

    /// a label used as hint for user of the state where
    /// they could get back
    fn label(&self) -> &'static str;

    /// display the state on screen and handle events until
    /// a transition must be done.
    ///
    /// This function can be called several times (if there's
    /// a back to a previous state, it's ran again).
    ///
    /// All drawing on screen should be controlled from there
    /// and all `con.w.flush` should be done only directly
    /// in this function.
    fn run(
        &mut self,
        con: &mut Context,
    ) -> Result<StateTransition>;

    /// provide a level (to the next state if it
    /// needs one which should come from the current one)
    fn get_level(
        &self,
        level_idx: usize,
    ) -> Option<Level>;
}

