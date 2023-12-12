#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

mod components;
mod plugins;
mod resources;
mod state;
mod util;

pub mod prelude {
    use super::*;
    pub use {components::*, plugins::*, resources::*, state::*, util::*};
}
