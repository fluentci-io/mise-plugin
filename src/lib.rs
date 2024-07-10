use extism_pdk::*;
use fluentci_pdk::dag;

#[plugin_fn]
pub fn setup() -> FnResult<String> {
    let stdout = dag()
        .mise()?
        .with_exec(vec!["mise", "--version"])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn install(args: String) -> FnResult<String> {
    let stdout = dag()
        .mise()?
        .with_exec(vec!["mise", "install", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn exec(args: String) -> FnResult<String> {
    let stdout = dag().mise()?.with_exec(vec![&args])?.stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn i(args: String) -> FnResult<String> {
    let stdout = dag()
        .mise()?
        .with_exec(vec!["mise", "install", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn x(args: String) -> FnResult<String> {
    let stdout = dag().mise()?.with_exec(vec![&args])?.stdout()?;
    Ok(stdout)
}
