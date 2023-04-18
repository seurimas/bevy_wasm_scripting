use bevy::{
    ecs::{
        query::ReadOnlyWorldQuery,
        system::{StaticSystemParam, SystemParam},
    },
    prelude::*,
};
use wasmer::{FromToNativeWasmType, NativeWasmTypeInto, WasmTypeList};

use crate::{resources::WasmScriptResource, WasmScript, WasmScriptComponent, WasmerStore};

/**
The `WasmScriptComponentEnv` is the primary entry point for running scripts associated with components
on entities.

It is a `SystemParam` with two generic types:
* First, the WasmScriptComponent whose script may get run.
* Second, an optional `ReadOnlyWorldQuery` which should consist of a tuple of `Without` query
elements. This should be filled with any components which are referenced by the system directly.

Within a system, the `call_if_instantiated` method can be used to execute an exported function.
*/
#[derive(SystemParam)]
pub struct WasmScriptComponentEnv<
    'w,
    's,
    WS: WasmScriptComponent,
    Without: ReadOnlyWorldQuery + 'static = (),
> {
    wasmer_store: ResMut<'w, WasmerStore>,
    assets: Res<'w, Assets<WasmScript>>,
    _used_components_query:
        Query<'w, 's, <WS as WasmScriptComponent>::ImportQueriedComponents, Without>,
    pub resources: StaticSystemParam<'w, 's, <WS as WasmScriptComponent>::ImportResources>,
}

/**
The `WasmScriptResourceEnv` is the primary entry point for running scripts associated with resources.

It is a `SystemParam` with two generic types:
* First, the WasmScriptResource whose script may get run.
* Second, an optional `ReadOnlyWorldQuery` which should consist of a tuple of `Without` query
elements. This should be filled with any components which are referenced by the system directly.

Within a system, the `call_if_instantiated` method can be used to execute an exported function.
*/
#[derive(SystemParam)]
pub struct WasmScriptResourceEnv<
    'w,
    's,
    WS: WasmScriptResource,
    Without: ReadOnlyWorldQuery + 'static = (),
> {
    wasmer_store: ResMut<'w, WasmerStore>,
    assets: Res<'w, Assets<WasmScript>>,
    _used_components_query:
        Query<'w, 's, <WS as WasmScriptResource>::ImportQueriedComponents, Without>,
    pub resources: StaticSystemParam<'w, 's, <WS as WasmScriptResource>::ImportResources>,
}

/**
The `WasmScriptEnv` is another entry point for running scripts, similar to `WasmScriptComponentEnv`.
It works for all scripts, and in situations where `WasmScriptComponentEnv` will not (multiple scripts
in one system, resource-based scripts).

SAFETY:
Unlike `WasmScriptComponentEnv`, there are no automatic references to any components or references.
Instead, any associated components or references should be added a system parameters manually.

Within a system, the `call_if_instantiated` method can be used to execute an exported function.
*/
#[derive(SystemParam)]
pub struct WasmScriptEnv<'w, 's> {
    wasmer_store: ResMut<'w, WasmerStore>,
    assets: StaticSystemParam<'w, 's, Res<'static, Assets<WasmScript>>>,
}

pub trait GeneralWasmScriptEnv {
    /**
    This will call the associated script's named function, with the provided arguments.

    If the associated script is not loaded or not fully instantiated, an error will be
    returned.

    Errors from the executed script function may also be returned.
    */
    fn call_if_instantiated_0<Rets: WasmTypeList>(
        &mut self,
        handle: &Handle<WasmScript>,
        function_name: &str,
    ) -> Result<Rets, anyhow::Error>;
    fn call_if_instantiated_1<S0: FromToNativeWasmType, Rets: WasmTypeList>(
        &mut self,
        handle: &Handle<WasmScript>,
        function_name: &str,
        s0: S0,
    ) -> Result<Rets, anyhow::Error>
    where
        S0::Native: NativeWasmTypeInto + FromToNativeWasmType;
    fn call_if_instantiated_2<
        S0: FromToNativeWasmType,
        S1: FromToNativeWasmType,
        Rets: WasmTypeList,
    >(
        &mut self,
        handle: &Handle<WasmScript>,
        function_name: &str,
        s0: S0,
        s1: S1,
    ) -> Result<Rets, anyhow::Error>
    where
        S0::Native: NativeWasmTypeInto + FromToNativeWasmType,
        S1::Native: NativeWasmTypeInto + FromToNativeWasmType;
    fn call_if_instantiated_3<
        S0: FromToNativeWasmType,
        S1: FromToNativeWasmType,
        S2: FromToNativeWasmType,
        Rets: WasmTypeList,
    >(
        &mut self,
        handle: &Handle<WasmScript>,
        function_name: &str,
        s0: S0,
        s1: S1,
        s2: S2,
    ) -> Result<Rets, anyhow::Error>
    where
        S0::Native: NativeWasmTypeInto + FromToNativeWasmType,
        S1::Native: NativeWasmTypeInto + FromToNativeWasmType,
        S2::Native: NativeWasmTypeInto + FromToNativeWasmType;
    fn call_if_instantiated_4<
        S0: FromToNativeWasmType,
        S1: FromToNativeWasmType,
        S2: FromToNativeWasmType,
        S3: FromToNativeWasmType,
        Rets: WasmTypeList,
    >(
        &mut self,
        handle: &Handle<WasmScript>,
        function_name: &str,
        s0: S0,
        s1: S1,
        s2: S2,
        s3: S3,
    ) -> Result<Rets, anyhow::Error>
    where
        S0::Native: NativeWasmTypeInto + FromToNativeWasmType,
        S1::Native: NativeWasmTypeInto + FromToNativeWasmType,
        S2::Native: NativeWasmTypeInto + FromToNativeWasmType,
        S3::Native: NativeWasmTypeInto + FromToNativeWasmType;
    fn call_if_instantiated_5<
        S0: FromToNativeWasmType,
        S1: FromToNativeWasmType,
        S2: FromToNativeWasmType,
        S3: FromToNativeWasmType,
        S4: FromToNativeWasmType,
        Rets: WasmTypeList,
    >(
        &mut self,
        handle: &Handle<WasmScript>,
        function_name: &str,
        s0: S0,
        s1: S1,
        s2: S2,
        s3: S3,
        s4: S4,
    ) -> Result<Rets, anyhow::Error>
    where
        S0::Native: NativeWasmTypeInto + FromToNativeWasmType,
        S1::Native: NativeWasmTypeInto + FromToNativeWasmType,
        S2::Native: NativeWasmTypeInto + FromToNativeWasmType,
        S3::Native: NativeWasmTypeInto + FromToNativeWasmType,
        S4::Native: NativeWasmTypeInto + FromToNativeWasmType;
}

macro_rules! impl_calls {
    ($call_name:ident $( $x:ident ),* ) => {
        #[allow(non_snake_case, unused_parens)]
        fn $call_name<$($x: FromToNativeWasmType,)* Rets: WasmTypeList>(
            &mut self,
            handle: &Handle<WasmScript>,
            function_name: &str,
            $( $x: $x, )*
        ) -> Result<Rets, anyhow::Error> where $($x::Native: FromToNativeWasmType + NativeWasmTypeInto,)* {
            self.assets
                .get(handle)
                .ok_or(anyhow::Error::msg("Asset not loaded"))
                .and_then(|script| {
                    if let WasmScript::Instantiated(_, instance) = script {
                        match instance
                                .exports
                                .get_function(function_name)
                                .map_err(anyhow::Error::new)
                                .and_then(|export| export.typed::<( $(<$x as FromToNativeWasmType>::Native),* ), Rets>(&mut self.wasmer_store.0).map_err(anyhow::Error::new)) {
                            Ok(exported) => exported
                                .call(&mut self.wasmer_store.0, $($x.to_native(),)*)
                                .map_err(anyhow::Error::new),
                            Err(err) => Err(anyhow::Error::msg(format!(
                                "{} is not exported correctly: {}",
                                function_name,
                                err
                            )))
                        }
                    } else {
                        Err(anyhow::Error::msg("Script not instantiated yet."))
                    }
                })
        }
    };
}

impl<'w, 's, WS: WasmScriptComponent, Without: ReadOnlyWorldQuery> GeneralWasmScriptEnv
    for WasmScriptComponentEnv<'w, 's, WS, Without>
{
    impl_calls!(call_if_instantiated_0);
    impl_calls!(call_if_instantiated_1 S0);
    impl_calls!(call_if_instantiated_2 S0, S1);
    impl_calls!(call_if_instantiated_3 S0, S1, S2);
    impl_calls!(call_if_instantiated_4 S0, S1, S2, S3);
    impl_calls!(call_if_instantiated_5 S0, S1, S2, S3, S4);
}

impl<'w, 's, WS: WasmScriptResource, Without: ReadOnlyWorldQuery> GeneralWasmScriptEnv
    for WasmScriptResourceEnv<'w, 's, WS, Without>
{
    impl_calls!(call_if_instantiated_0);
    impl_calls!(call_if_instantiated_1 S0);
    impl_calls!(call_if_instantiated_2 S0, S1);
    impl_calls!(call_if_instantiated_3 S0, S1, S2);
    impl_calls!(call_if_instantiated_4 S0, S1, S2, S3);
    impl_calls!(call_if_instantiated_5 S0, S1, S2, S3, S4);
}

impl<'w, 's> GeneralWasmScriptEnv for WasmScriptEnv<'w, 's> {
    impl_calls!(call_if_instantiated_0);
    impl_calls!(call_if_instantiated_1 S0);
    impl_calls!(call_if_instantiated_2 S0, S1);
    impl_calls!(call_if_instantiated_3 S0, S1, S2);
    impl_calls!(call_if_instantiated_4 S0, S1, S2, S3);
    impl_calls!(call_if_instantiated_5 S0, S1, S2, S3, S4);
}
