use crate::prims::perf_data::PerfDataValue::JavaLong;


use std::collections::HashMap;
use std::rc::Rc;

#[derive(PartialEq)]
pub enum Variability {
    Constant = 1,
    Monotonic = 2,
    Variable = 3,
}

impl From<i32> for Variability {
    fn from(v: i32) -> Self {
        match v {
            1 => Variability::Constant,
            2 => Variability::Monotonic,
            3 => Variability::Variable,
            _ => panic!("unexpected variability value: {}", v),
        }
    }
}

impl From<i64> for Variability {
    fn from(v: i64) -> Self {
        match v {
            1 => Variability::Constant,
            2 => Variability::Monotonic,
            3 => Variability::Variable,
            _ => panic!("unexpected variability value: {}", v),
        }
    }
}

pub enum Units {
    None = 1,
    Bytes = 2,
    Ticks = 3,
    Events = 4,
    String = 5,
    Hertz = 6,
}

impl From<i32> for Units {
    fn from(v: i32) -> Self {
        match v {
            1 => Units::None,
            2 => Units::Bytes,
            3 => Units::Ticks,
            4 => Units::Events,
            5 => Units::String,
            6 => Units::Hertz,
            _ => panic!("unexpected units value: {}", v),
        }
    }
}

pub enum Flags {
    None = 0x0,
    Supported = 0x1, // interface is supported - java.* and com.sun.*
}

pub struct PerfDataManager {
    all: Option<PerfDataList>,
    sampled: Option<PerfDataList>,
    constants: Option<PerfDataList>,
}

static mut PERF_DATA_MANAGER: Option<PerfDataManager> = None;

impl PerfDataManager {
    fn new() -> PerfDataManager {
        return PerfDataManager {
            all: None,
            sampled: None,
            constants: None,
        };
    }

    fn instance() {
        unsafe {
            if PERF_DATA_MANAGER.is_none() {
                PERF_DATA_MANAGER = Some(Self::new());
            }
        }
    }

    pub fn get_instance() -> &'static PerfDataManager {
        Self::instance();
        unsafe {
            return PERF_DATA_MANAGER.as_ref().unwrap();
        }
    }

    pub fn get_mut_instance() -> &'static mut PerfDataManager {
        Self::instance();
        unsafe {
            return PERF_DATA_MANAGER.as_mut().unwrap();
        }
    }

    pub fn exists(&self, name: &str) -> bool {
        if self.all.is_none() {
            return false;
        }
        return self.all.as_ref().unwrap().contains(name);
    }

    pub fn add_item(&mut self, p: Rc<PerfData>, sampled: bool) {
        if self.all.is_none() {
            self.all = Some(PerfDataList::new(100));
        }
        assert!(
            !self.all.as_ref().unwrap().contains(p.name.as_str()),
            "duplicate name added"
        );
        self.all.as_mut().unwrap().append(p.clone());
        if p.variability == Variability::Constant {
            if self.constants.is_none() {
                self.constants = Some(PerfDataList::new(25));
            }
            self.constants.as_mut().unwrap().append(p);
            return;
        }
        if sampled {
            if self.sampled.is_none() {
                self.sampled = Some(PerfDataList::new(25));
            }
            self.sampled.as_mut().unwrap().append(p);
        }
    }

    pub fn create_long_constant(
        &mut self,
        name_utf: &str,
        units: Units,
        value: i64,
    ) -> Rc<PerfData> {
        let p = Rc::new(PerfData::new_long_constant(name_utf, units, value));
        self.add_item(p.clone(), false);
        return p;
    }

    pub fn create_long_counter(
        &mut self,
        name_utf: &str,
        units: Units,
        value: i64,
    ) -> Rc<PerfData> {
        let p = Rc::new(PerfData::new_long_counter(name_utf, units, value));
        self.add_item(p.clone(), false);
        return p;
    }

    pub fn create_long_variable(
        &mut self,
        name_utf: &str,
        units: Units,
        value: i64,
    ) -> Rc<PerfData> {
        let p = Rc::new(PerfData::new_long_variable(name_utf, units, value));
        self.add_item(p.clone(), false);
        return p;
    }
}

struct PerfDataList {
    data_array: Vec<Rc<PerfData>>,
    cache: HashMap<String, usize>,
}

impl PerfDataList {
    pub fn new(length: usize) -> PerfDataList {
        return PerfDataList {
            data_array: Vec::with_capacity(length),
            cache: Default::default(),
        };
    }

    pub fn find_by_name(&self, name: &str) -> Option<Rc<PerfData>> {
        let index = self.cache.get(name);
        if index.is_none() {
            return None;
        }
        return self.at(*index.unwrap());
    }

    pub fn contains(&self, name: &str) -> bool {
        return self.find_by_name(name).is_some();
    }

    pub fn length(&self) -> usize {
        return self.data_array.len();
    }

    pub fn append(&mut self, data: Rc<PerfData>) {
        self.data_array.push(data.clone());
        self.cache
            .insert(data.name.clone(), self.data_array.len() - 1);
    }

    pub fn remove(&mut self, data: Rc<PerfData>) {
        let index = self.cache.get(data.name.as_str());
        if index.is_some() {
            self.data_array.remove(*index.unwrap());
        }
    }

    pub fn at(&self, index: usize) -> Option<Rc<PerfData>> {
        if index >= self.data_array.len() {
            return Some(self.data_array[index].clone());
        }
        return None;
    }
}

pub struct PerfData {
    name: String,
    variability: Variability,
    units: Units,
    //on_c_heap:bool
    flags: Flags,
    value: PerfDataValue,
}

impl PerfData {
    fn new_long_constant(name_utf: &str, units: Units, value: i64) -> PerfData {
        return PerfData {
            name: name_utf.to_string(),
            variability: Variability::Constant,
            units,
            flags: Flags::None,
            value: JavaLong(value),
        };
    }

    fn new_long_counter(name_utf: &str, units: Units, value: i64) -> PerfData {
        return PerfData {
            name: name_utf.to_string(),
            variability: Variability::Monotonic,
            units,
            flags: Flags::None,
            value: JavaLong(value),
        };
    }

    fn new_long_variable(name_utf: &str, units: Units, value: i64) -> PerfData {
        return PerfData {
            name: name_utf.to_string(),
            variability: Variability::Variable,
            units,
            flags: Flags::None,
            value: JavaLong(value),
        };
    }

    pub fn get_address(&self) -> usize {
        let p = self as *const PerfData;
        return p as usize;
    }
}

enum PerfDataValue {
    JavaLong(i64),
}

pub enum CounterNS {
    // top level name spaces
    JavaNs,
    ComNs,
    SunNs,
    // subsystem name spaces
    JavaGc, // Garbage Collection name spaces
    ComGc,
    SunGc,
    JavaCi, // Compiler name spaces
    ComCi,
    SunCi,
    JavaCls, // Class Loader name spaces
    ComCls,
    SunCls,
    JavaRt, // Runtime name spaces
    ComRt,
    SunRt,
    JavaOs, // Operating System name spaces
    ComOs,
    SunOs,
    JavaThreads, // Threads System name spaces
    ComThreads,
    SunThreads,
    JavaProperty, // Java Property name spaces
    ComProperty,
    SunProperty,
    NullNs,
}
