use bevy::{
    ecs::{
        query::ReadOnlyWorldQuery,
        system::{StaticSystemParam, SystemParam},
    },
    prelude::*,
};
use wasmer::FromToNativeWasmType;

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
    _used_components_query: Query<'w, 's, WS::ImportQueriedComponents, Without>,
    pub resources: StaticSystemParam<'w, 's, WS::ImportResources>,
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
    _used_components_query: Query<'w, 's, WS::ImportQueriedComponents, Without>,
    pub resources: StaticSystemParam<'w, 's, WS::ImportResources>,
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
    fn call_if_instantiated<Args: FromToNativeWasmType, Rets: FromToNativeWasmType>(
        &mut self,
        handle: &Handle<WasmScript>,
        function_name: &str,
        args: Args,
    ) -> Result<Rets, anyhow::Error>;
}

impl<'w, 's, WS: WasmScriptComponent, Without: ReadOnlyWorldQuery> GeneralWasmScriptEnv
    for WasmScriptComponentEnv<'w, 's, WS, Without>
{
    fn call_if_instantiated<Args: FromToNativeWasmType, Rets: FromToNativeWasmType>(
        &mut self,
        handle: &Handle<WasmScript>,
        function_name: &str,
        args: Args,
    ) -> Result<Rets, anyhow::Error> {
        (&mut self.wasmer_store, &self.assets).call_if_instantiated(handle, function_name, args)
    }
}

impl<'w, 's, WS: WasmScriptResource, Without: ReadOnlyWorldQuery> GeneralWasmScriptEnv
    for WasmScriptResourceEnv<'w, 's, WS, Without>
{
    fn call_if_instantiated<Args: FromToNativeWasmType, Rets: FromToNativeWasmType>(
        &mut self,
        handle: &Handle<WasmScript>,
        function_name: &str,
        args: Args,
    ) -> Result<Rets, anyhow::Error> {
        (&mut self.wasmer_store, &self.assets).call_if_instantiated(handle, function_name, args)
    }
}

impl<'w, 's> GeneralWasmScriptEnv for WasmScriptEnv<'w, 's> {
    fn call_if_instantiated<Args: FromToNativeWasmType, Rets: FromToNativeWasmType>(
        &mut self,
        handle: &Handle<WasmScript>,
        function_name: &str,
        args: Args,
    ) -> Result<Rets, anyhow::Error> {
        (&mut self.wasmer_store, std::ops::Deref::deref(&self.assets)).call_if_instantiated(
            handle,
            function_name,
            args,
        )
    }
}

impl<'w> GeneralWasmScriptEnv for (&mut ResMut<'w, WasmerStore>, &Res<'w, Assets<WasmScript>>) {
    fn call_if_instantiated<Args: FromToNativeWasmType, Rets: FromToNativeWasmType>(
        &mut self,
        handle: &Handle<WasmScript>,
        function_name: &str,
        args: Args,
    ) -> Result<Rets, anyhow::Error> {
        self.1
            .get(handle)
            .ok_or(anyhow::Error::msg("Asset not loaded"))
            .and_then(|script| {
                if let WasmScript::Instantiated(_, instance) = script {
                    if let Some(exported) = instance
                        .exports
                        .get_function(function_name)
                        .ok()
                        .and_then(|export| export.typed::<Args, Rets>(&mut self.0 .0).ok())
                    {
                        exported
                            .call(&mut self.0 .0, args)
                            .map_err(anyhow::Error::new)
                    } else {
                        Err(anyhow::Error::msg(format!(
                            "{} is not exported correctly.",
                            function_name
                        )))
                    }
                } else {
                    Err(anyhow::Error::msg("Script not instantiated yet."))
                }
            })
    }
}
