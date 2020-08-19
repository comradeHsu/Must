use crate::oops::object::Object;

pub struct Universe {
    system_thread_group: Option<Object>,
    main_thread_group: Option<Object>,
}

static mut UNIVERSE: Universe = Universe::new();

impl Universe {

    const fn new() -> Universe {
        return Universe {
            system_thread_group: None,
            main_thread_group: None
        }
    }

    pub fn set_system_thread_group(system_thread_group: Option<Object>) {
        unsafe {
            UNIVERSE.system_thread_group = system_thread_group
        }
    }

    pub fn set_main_thread_group(main_thread_group: Option<Object>) {
        unsafe {
            UNIVERSE.main_thread_group = main_thread_group
        }
    }
}