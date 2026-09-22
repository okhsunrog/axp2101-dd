// Included twice (once per `bisync` module). The generated code exposes both
// `read`/`read_async` etc., so these thin wrappers pick the right one per flavour.
// Register operations are consumed by their read/write/modify methods, so the
// helpers take them by value.

#[allow(dead_code)]
#[only_sync]
fn read_internal<'b, Block, Register, Access>(
    op: device_driver::RegisterOperation<'b, Block, Register, u8, Access, ()>,
) -> Result<Register, <Block::Interface as RegisterInterfaceBase>::Error>
where
    Block: device_driver::Block,
    Block::Interface: RegisterInterface<AddressType = u8>,
    Register: device_driver::Fieldset,
    Access: device_driver::ReadCapability,
{
    op.read()
}

#[allow(dead_code)]
#[only_async]
async fn read_internal<'b, Block, Register, Access>(
    op: device_driver::RegisterOperation<'b, Block, Register, u8, Access, ()>,
) -> Result<Register, <Block::Interface as RegisterInterfaceBase>::Error>
where
    Block: device_driver::Block,
    Block::Interface: RegisterInterface<AddressType = u8>,
    Register: device_driver::Fieldset,
    Access: device_driver::ReadCapability,
{
    op.read_async().await
}

#[allow(dead_code)]
#[only_sync]
fn write_internal<'b, Block, Register, Access>(
    op: device_driver::RegisterOperation<'b, Block, Register, u8, Access, ()>,
    f: impl FnOnce(&mut Register),
) -> Result<(), <Block::Interface as RegisterInterfaceBase>::Error>
where
    Block: device_driver::Block,
    Block::Interface: RegisterInterface<AddressType = u8>,
    Register: device_driver::Fieldset,
    Access: device_driver::WriteCapability,
{
    op.write(f)
}

#[allow(dead_code)]
#[only_async]
async fn write_internal<'b, Block, Register, Access>(
    op: device_driver::RegisterOperation<'b, Block, Register, u8, Access, ()>,
    f: impl FnOnce(&mut Register),
) -> Result<(), <Block::Interface as RegisterInterfaceBase>::Error>
where
    Block: device_driver::Block,
    Block::Interface: RegisterInterface<AddressType = u8>,
    Register: device_driver::Fieldset,
    Access: device_driver::WriteCapability,
{
    op.write_async(f).await
}

#[allow(dead_code)]
#[only_sync]
fn modify_internal<'b, Block, Register, Access>(
    op: device_driver::RegisterOperation<'b, Block, Register, u8, Access, ()>,
    f: impl FnOnce(&mut Register),
) -> Result<(), <Block::Interface as RegisterInterfaceBase>::Error>
where
    Block: device_driver::Block,
    Block::Interface: RegisterInterface<AddressType = u8>,
    Register: device_driver::Fieldset,
    Access: device_driver::ReadCapability + device_driver::WriteCapability,
{
    op.modify(f)
}

#[allow(dead_code)]
#[only_async]
async fn modify_internal<'b, Block, Register, Access>(
    op: device_driver::RegisterOperation<'b, Block, Register, u8, Access, ()>,
    f: impl FnOnce(&mut Register),
) -> Result<(), <Block::Interface as RegisterInterfaceBase>::Error>
where
    Block: device_driver::Block,
    Block::Interface: RegisterInterface<AddressType = u8>,
    Register: device_driver::Fieldset,
    Access: device_driver::ReadCapability + device_driver::WriteCapability,
{
    op.modify_async(f).await
}
