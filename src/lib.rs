#![allow(non_snake_case)]
use std::any::Any;
use std::sync::Arc;

use std::collections::HashMap;
pub trait Handler<R, P> {
    fn call(self, param: R);
}

pub trait FromParams<T> {
    fn extract(req: &T) -> &Self;
}

#[derive(Default)]
pub struct TypeSet {
    map: HashMap<String, Box<dyn Any + Send + Sync>>,
}

pub struct EngineState {
    data: TypeSet,
}

#[derive(Default)]
pub struct EngineBuilder {
    data: TypeSet,
}

impl EngineBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_state<T: 'static + Send + Sync>(mut self, val: T) -> Self {
        self.data
            .map
            .insert(std::any::type_name::<T>().to_string(), Box::new(val));
        self
    }

    pub fn build(self) -> Engine {
        let state = Arc::new(EngineState { data: self.data });
        Engine { state }
    }
}

#[derive(Clone)]
pub struct Engine {
    state: Arc<EngineState>,
}

impl Engine {
    pub fn run<P>(self, f: impl Handler<Arc<EngineState>, P>) -> Self {
        f.call(self.state.clone());
        self
    }
}

#[macro_export]
macro_rules! all_tuples {
    ($name:ident) => {
        $name!([], T1);
        $name!([T1], T2);
        $name!([T1, T2], T3);
        $name!([T1, T2, T3], T4);
        $name!([T1, T2, T3, T4], T5);
        $name!([T1, T2, T3, T4, T5], T6);
        $name!([T1, T2, T3, T4, T5, T6], T7);
        $name!([T1, T2, T3, T4, T5, T6, T7], T8);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8], T9);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T10], T11);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11], T12);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12], T13);
        $name!(
            [T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T13, T14],
            T15
        );
        $name!(
            [T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T13, T14, T15],
            T16
        );
    };
}

macro_rules! impl_handler {
    (
        [$($ty:ident),*], $last:ident
    ) => {
        impl<F, R, $($ty,)* $last> Handler<R, ($($ty,)* $last,)> for F
        where
            F: FnOnce($(&$ty, )* &$last),
            R: Clone,
            $($ty: FromParams<R>,)*
            $last: FromParams<R>,
        {
            fn call(self, req: R) {
                $(let $ty = FromParams::extract(&req);)*
                let $last = FromParams::extract(&req);
                (self)($($ty,)* $last,);
            }
        }
    };
}

all_tuples!(impl_handler);

impl<T: 'static> FromParams<Arc<EngineState>> for T {
    fn extract(req: &Arc<EngineState>) -> &Self {
        let type_name = std::any::type_name::<Self>();
        let boxed = req.data.map.get(type_name).unwrap_or_else(|| panic!("Type {type_name} not found in EngineState. Make sure to register it using EngineBuilder::with()."));

        boxed.downcast_ref::<Self>().unwrap_or_else(|| {
            panic!("Type {type_name} found in EngineState but failed to downcast.")
        })
    }
}
