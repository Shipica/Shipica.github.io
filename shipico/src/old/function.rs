macro_rules! functions {
    ($(fn $func_name:ident($($param:ident : $param_type:tt),*) -> ($($ret:ident : $ret_type:tt),*) $body: stmt )*) => {
        ::paste::paste! {
            use crate::params::*;

            #[derive(Clone, Debug)]
            pub struct FunctionDefinition {
                pub inputs: &'static [ParamType],
                pub outputs: &'static [ParamType],
                pub name: &'static str
            }

            pub const FUNCTIONS: &[FunctionDefinition] = &[
                $(
                    FunctionDefinition {
                        inputs: &[
                            $(
                                ParamType::$param_type
                            ),*
                        ],
                        outputs: &[
                            $(
                                ParamType::$ret_type
                            ),*
                        ],
                        name: stringify!($func_name),
                    }
                ),*
            ];

            impl FunctionDefinition {
                fn call(
                    &self,
                    input_addresses: &[usize],
                    memory: &mut Vec<Param>
                ) -> (usize, usize) {
                    assert!(self.inputs.len() == input_addresses.len());

                    let output_addresses = (memory.len(), memory.len() + self.outputs.len());
                    match self.name {
                        $(
                            stringify!($func_name) => {
                                // extracting nessessary inputs from memory
                                let mut input_index = 0;
                                $(
                                    let [<input_ $param>] = memory[input_addresses[input_index]].clone();
                                    input_index += 1;
                                )*
                                let output = $func_name(
                                    $([<input_ $param>].[<into_ $param_type>]().unwrap()),*
                                );
                                let ($([<output_ $ret>]),*) = output;

                                $(
                                    memory.push(Param::from([<output_ $ret>]));
                                )*
                            }
                        ),*,
                        _ => unreachable!()
                    }
                    output_addresses
                }
            }

            $(
                #[allow(unused_parens)]
                pub fn $func_name($($param:$param_type),*) -> ($($ret_type),*) {
                    $body
                }
            )*
        }
    };
}

// trait Function {
//     // const INPUT: &'static [ParamType];
//     // const OUTPUT: &'static [ParamType];
//     // This is required to be as function
//     // I can't just use associated consts,
//     // because this way it would not be possible
//     // to use Function trait as trait object.
//     fn function_inputs(&self) -> &'static [ParamType];
//     fn function_outputs(&self) -> &'static [ParamType];
//     fn exec(&self, input_addresses: &[usize], memory: &mut Vec<Param>) -> (usize, usize);
// }

//     fn exec(&self, input_addresses: &[usize], memory: &mut Stack) -> Vec<usize> {
//         assert_eq!(input_addresses.len(), Self::INPUT.len());

//         let mut output_addresses = Vec::with_capacity(Self::OUTPUT.len());

//         let mut inputs = vec![];
//         for (i, addr) in input_addresses.iter().enumerate() {
//             let input = memory._inner[*addr].clone();
//             assert_eq!(std::mem::discriminant(&input), Self::INPUT[i]);
//             inputs.push(input);
//         }

//         let param = &memory._inner[input_addresses[0]];

//         output_addresses.push(memory._inner.len());
//         let output = (self)(inputs[0].into_i32().unwrap());
//         memory._inner.push(Param::from_f32(output));

//         output_addresses
//     }

functions!(
    fn asd(param1: i64, param2: f32) -> (
        result: f32
    ) {
        return (param2);
    }

    fn foo(param3: f64) -> () {

    }
);

pub fn aaaa() {
    asd(5, 2.0);
}
// trait Function {
//     const INPUT_SIZE: usize;
//     const OUTPUT_SIZE: usize;
//     const INPUTS: [ParamType; Self::INPUT_SIZE];
//     const OUTPUTS: [ParamType; Self::OUTPUT_SIZE];

//     fn exec(
//         &self,
//         input_addresses: [usize; Self::INPUT_SIZE],
//         memory: &mut Vec<Param>,
//     ) -> (usize, usize);
// }

// struct A {}

// impl Function for A {
//     const INPUT_SIZE: usize = 1;

//     const OUTPUT_SIZE: usize = 1;

//     const INPUTS: [ParamType; Self::INPUT_SIZE] = [ParamType::i64];

//     const OUTPUTS: [ParamType; Self::OUTPUT_SIZE] = [ParamType::i64];

//     fn exec(
//         &self,
//         input_addresses: [usize; Self::INPUT_SIZE],
//         memory: &mut Vec<Param>,
//     ) -> (usize, usize) {
//         (0, 0)
//     }
// }
//
