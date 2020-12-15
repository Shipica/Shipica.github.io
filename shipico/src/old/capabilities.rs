// use std::marker::PhantomData;

// use crate::math::Vec2;

// macro_rules! _cap_indices {
//     ($u: literal, $x:tt) => {
//         paste::paste!{
//             const [<$x:upper _INDEX>]: usize = $u;
//         }
//         const CAPABILITIES_LEN: usize = $u + 1usize;
//     };
//     ($u: literal, $x:tt, $($xs:tt),*) => {
//         paste::paste!{
//             const [<$x:upper _INDEX>]: usize = $u;
//         }
//         _cap_indices!($u + 1usize, $($xs),*);
//     }
// }

// macro_rules! capabilities {
//     ($($cap: ident : fn($($param_type:pat),*)),*) => {
//         _cap_indices!(0usize, $($cap),*);

//         paste::paste! {
//             pub trait Capability: seal::Sealed {
//                 const INDEX: usize;
//                 // type FnType: Copy + Sized;
//             }

//             $(
//                 pub struct $cap;
//                 // impl<T> Capability<T> for $cap {
//                 impl Capability for $cap {
//                     const INDEX: usize = [<$cap:upper _INDEX>];
//                     // type FnType = fn($($param_type),*);
//                 }
//             )*
//             mod seal {
//                 pub trait Sealed {}
//                 $(
//                     impl Sealed for super::$cap {}
//                 )*
//             }

//             pub struct Capabilities<T> {
//                 hm: [*const (); CAPABILITIES_LEN],
//                 _owner: PhantomData<T>,
//             }

//             impl<T> Capabilities<T> {
//                 $(
//                     pub fn [<get_ $cap:lower>](&self) -> Option<fn($($param_type),*)> {
//                         let maybe_fn = self.hm[[<$cap:upper _INDEX>]];
//                         unsafe { maybe_fn.cast::<fn($($param_type),*)>().as_ref().map(|x| *x) }
//                     }

//                     pub unsafe fn [<get_ $cap:lower _uncheched>](&self) -> fn($($param_type),*) {
//                         let maybe_fn = self.hm[[<$cap:upper _INDEX>]];
//                         *maybe_fn.cast::<fn($($param_type),*)>()
//                     }
//                 )*

//                 // pub fn get<C: Capability<T>>(&self) -> Option<C::FnType> {
//                 //     let maybe_fn = self.hm[C::INDEX];
//                 //     unsafe { maybe_fn.cast::<C::FnType>().as_ref().map(|x| *x) }
//                 // }

//                 // pub unsafe fn get_unchecked<C: Capability<T>>(&self) -> C::FnType {
//                 //     let maybe_fn = self.hm[C::INDEX];
//                 //     *maybe_fn.cast::<C::FnType>()
//                 // }
//             }

//             #[derive(Clone, Copy)]
//             pub struct CapabilitiesBuilder<T> {
//                 hm: [*const (); CAPABILITIES_LEN],
//                 _owner: PhantomData<T>,
//             }

//             impl<T> CapabilitiesBuilder<T> {
//                 pub const fn new() -> CapabilitiesBuilder<T>{
//                     CapabilitiesBuilder {
//                         hm: [std::ptr::null() as *const (); CAPABILITIES_LEN],
//                         _owner: PhantomData,
//                     }
//                 }
//                 $(
//                     pub const fn [<with_ $cap:lower>](mut self, implementation: fn($($param_type),*)) -> Self{
//                         self.hm[[<$cap:upper _INDEX>]] = implementation as *const ();
//                         self
//                     }
//                 )*

//                 pub const fn build(self) -> Capabilities<T> {
//                     Capabilities {
//                         hm: self.hm,
//                         _owner: self._owner,
//                     }
//                 }
//             }
//         }
//     }
// }

// capabilities!(Drag: fn(&mut T, Vec2));

// // impl<T> Capabilities<T> {
// //     pub fn get<C: Capability>(&self) -> Option<C::FnType> {
// //         let maybe_fn = self.hm[C::INDEX];
// //         unsafe { maybe_fn.cast::<C::FnType>().as_ref().map(|x| *x) }
// //     }

// //     pub unsafe fn get_unchecked<C: Capability>(&self) -> C::FnType {
// //         let maybe_fn = self.hm[C::INDEX];
// //         *maybe_fn.cast::<C::FnType>()
// //     }
// // }

// pub struct OwnerOf<'a, T>
// where
//     T: Capability,
// {
//     owner: &'a dyn std::any::Any,
//     _ph: PhantomData<T>,
// }

// pub trait UiElement: Sized {
//     const CAPABILITIES: Capabilities<Self>;
// }

// struct NodeData {}

// impl NodeData {
//     fn drag(&mut self, delta: Vec2) {}
// }

// impl UiElement for NodeData {
//     const CAPABILITIES: Capabilities<Self> =
//         CapabilitiesBuilder::new().with_drag(NodeData::drag).build();
// }

// fn a() {
//     let mut data = NodeData {};
//     data.drag([0.0, 0.0].into())
// }
