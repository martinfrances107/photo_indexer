pub(crate) mod button;
pub(crate) mod pannel;

#[derive(Clone, Debug)]
pub enum SideBarState {
    Open,
    Close,
}

impl SideBarState {
    // Used to dynamically control class the settings bar.
    const fn is_hidden(&self) -> bool {
        match self {
            Self::Open => false,
            Self::Close => true,
        }
    }

    const fn toggle(&self) -> Self {
        match self {
            Self::Open => Self::Close,
            Self::Close => Self::Open,
        }
    }
}
