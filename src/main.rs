use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    agent_papercut::run()
}
