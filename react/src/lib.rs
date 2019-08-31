use std::collections::{BTreeSet, HashMap};
use std::iter::FromIterator;

/// `InputCellID` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InputCellID(usize);
/// `ComputeCellID` is a unique identifier for a compute cell.
/// Values of type `InputCellID` and `ComputeCellID` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellID = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellID = r.create_compute(&[react::CellID::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComputeCellID(usize);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CallbackID(usize);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellID {
    Input(InputCellID),
    Compute(ComputeCellID),
}

impl CellID {
    fn idx(&self) -> usize {
        match self {
            CellID::Input(id) => id.0,
            CellID::Compute(id) => id.0
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

pub struct Reactor<'a, T> {
    data: Vec<Cell<'a, T>>,
    compute_info: HashMap<usize, ComputeInfo<'a, T>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Reactor {
            data: Vec::new(),
            compute_info: HashMap::new(),
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellID {
        let id = self.data.len();
        self.data.push(Cell::new(initial));
        InputCellID(id)
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + 'a>(
        &mut self,
        dependencies: &[CellID],
        compute_func: F,
    ) -> Result<ComputeCellID, CellID> {
        let new_id = self.data.len();
        for id in dependencies {
            if id.idx() >= new_id {
                return Err(*id);
            }
        }

        let mut cells: Vec<usize> = Vec::with_capacity(dependencies.len());
        for id in dependencies {
            let idx = id.idx();
            cells.push(idx);
            let cell = &mut self.data[idx];
            cell.add_downstream(new_id);
        };
        self.compute_info.insert(new_id, ComputeInfo {
            compute_func: Box::new(compute_func),
            dependencies: cells,
        });
        let init = self.compute(new_id);
        self.data.push(Cell::new(init));
        Ok(ComputeCellID(new_id))
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellID) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellID) -> Option<T> {
        self.data.get(id.idx()).map(|cell| cell.value)
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellID) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellID, new_value: T) -> bool {
        let exists = self.data.get_mut(id.0).map(|cell| {
            cell.value = new_value;
            true
        }).unwrap_or(false);
        if exists {
            self.propogate(id.0);
        }
        exists
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) -> () + 'a>(
        &mut self,
        id: ComputeCellID,
        callback: F,
    ) -> Option<CallbackID> {
        self.data.get_mut(id.0).map(|cell| cell.add_callback(callback))
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellID,
        callback: CallbackID,
    ) -> Result<(), RemoveCallbackError> {
        self.data.get_mut(cell.0)
            .ok_or(RemoveCallbackError::NonexistentCell)
            .and_then(|cell| cell.remove_callback(callback))
    }

    fn compute(&self, idx: usize) -> T {
        let ComputeInfo { ref compute_func, ref dependencies } = self.compute_info[&idx];
        let inputs: Vec<T> = dependencies.iter().map(|&i| self.data[i].value).collect();
        compute_func(&inputs)
    }

    fn propogate(&mut self, start_idx: usize) {
        // FIXME: this allocation seems unnecessary, but I don't
        // know how else to get around lifetime errors
        let mut downstreams: BTreeSet<usize> = BTreeSet::from_iter(self.data[start_idx].downstreams.iter().cloned());
        while let Some(&cell_id) = downstreams.iter().next() {
            let value = self.compute(cell_id);
            let cell = &mut self.data[cell_id];
            let oldvalue = cell.value;
            cell.value = value;
            if oldvalue != value {
                cell.run_callbacks();
                downstreams.extend(&cell.downstreams);
            }
            downstreams.remove(&cell_id);
        }
    }
}

struct Cell<'a, T> {
    value: T,
    downstreams: Vec<usize>,
    counter: usize,
    callbacks: Vec<Callback<'a, T>>,
}

struct Callback<'a, T>(usize, Box<FnMut(T) -> ()+ 'a>);

impl<'a, T: Copy> Cell<'a, T> {
    fn new(value: T) -> Self {
        Cell {
            value,
            downstreams: Vec::new(),
            counter: 0,
            callbacks: Vec::new(),
        }
    }

    fn add_downstream(&mut self, downstream: usize) {
        self.downstreams.push(downstream);
    }

    fn add_callback<F: FnMut(T) -> () + 'a>(&mut self, callback: F) -> CallbackID {
        let id = self.counter;
        self.counter += 1;
        self.callbacks.push(Callback(id, Box::new(callback)));
        CallbackID(id)
    }

    fn remove_callback(&mut self, id: CallbackID) -> Result<(), RemoveCallbackError> {
        if let Some(idx) = self.callbacks.iter().enumerate().find(|(_, cb)| cb.0 == id.0).map(|(i, _)| i) {
            self.callbacks.remove(idx);
            Ok(())
        } else {
            Err(RemoveCallbackError::NonexistentCallback)
        }
    }

    fn run_callbacks(&mut self) {
        for callback in self.callbacks.iter_mut() {
            callback.1(self.value);
        }
    }
}


struct ComputeInfo<'a, T> {
    compute_func: Box<Fn(&[T]) -> T + 'a>,
    dependencies: Vec<usize>,
}