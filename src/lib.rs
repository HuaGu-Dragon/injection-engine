#![allow(non_snake_case)]
use std::sync::Arc;

use std::collections::HashMap;
pub trait Handler<R, P> {
    fn call(self, param: R);
}

pub trait FromParams<T> {
    fn extract(req: &T) -> Self;
}

pub trait FromParamsOwn<T> {
    fn extract(req: T) -> Self;
}

#[derive(Default)]
pub struct TypeSet {
    map: HashMap<String, Box<dyn std::any::Any>>,
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

    pub fn with<T: 'static + Send + Sync>(mut self, val: T) -> Self {
        self.data
            .map
            .insert(std::any::type_name::<T>().to_string(), Box::new(val));
        self
    }

    pub fn build(self) -> Arc<EngineState> {
        Arc::new(EngineState { data: self.data })
    }
}

#[derive(Clone)]
pub struct Engine {
    state: Arc<EngineState>,
}

impl Engine {
    pub fn run<P>(self, f: impl Handler<Arc<EngineState>, P>) {
        f.call(self.state.clone());
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
            F: FnOnce($($ty, )* $last),
            $($ty: FromParams<R>,)*
            $last: FromParamsOwn<R>,
        {
            fn call(self, req: R) {
                $(let $ty = FromParams::extract(&req);)*
                let $last = FromParamsOwn::extract(req);
                (self)($($ty,)* $last,);
            }
        }
    };
}

all_tuples!(impl_handler);

macro_rules! impl_params {
    ($($ty:ident),*) => {
        $(
            impl FromParamsOwn<Arc<EngineState>> for $ty {
                fn extract(req: Arc<EngineState>) -> Self {
                    todo!()
                }
            }
        )*
    };
}

impl_params!(
    i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, f32, f64, bool, char, String
);
