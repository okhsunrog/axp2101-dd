#[allow(dead_code)]
#[only_sync]
fn read_internal<'a, Interface, Register, Access>(
    op: &mut device_driver::RegisterOperation<'a, Interface, u8, Register, Access>
) -> Result<Register, Interface::Error>
where
    Interface: RegisterInterface<AddressType = u8>,
    Register: device_driver::FieldSet,
    Access: device_driver::ReadCapability,
{
    op.read()
}

#[allow(dead_code)]
#[only_async]
async fn read_internal<'a, Interface, Register, Access>(
    op: &mut device_driver::RegisterOperation<'a, Interface, u8, Register, Access>
) -> Result<Register, Interface::Error>
where
    Interface: RegisterInterface<AddressType = u8>,
    Register: device_driver::FieldSet,
    Access: device_driver::ReadCapability,
{
    op.read_async().await
}

#[allow(dead_code)]
#[only_sync]
fn write_internal<'a, Interface, Register, Access, R>(
    op: &mut device_driver::RegisterOperation<'a, Interface, u8, Register, Access>,
    f: impl FnOnce(&mut Register) -> R,
) -> Result<R, Interface::Error>
where
    Interface: RegisterInterface<AddressType = u8>,
    Register: device_driver::FieldSet,
    Access: device_driver::WriteCapability,
{
    op.write(f)
}

#[allow(dead_code)]
#[only_async]
async fn write_internal<'a, Interface, Register, Access, R>(
    op: &mut device_driver::RegisterOperation<'a, Interface, u8, Register, Access>,
    f: impl FnOnce(&mut Register) -> R,
) -> Result<R, Interface::Error>
where
    Interface: RegisterInterface<AddressType = u8>,
    Register: device_driver::FieldSet,
    Access: device_driver::WriteCapability,
{
    op.write_async(f).await
}

#[allow(dead_code)]
#[only_sync]
fn modify_internal<'a, Interface, Register, Access, R>(
    op: &mut device_driver::RegisterOperation<'a, Interface, u8, Register, Access>,
    f: impl FnOnce(&mut Register) -> R,
) -> Result<R, Interface::Error>
where
    Interface: RegisterInterface<AddressType = u8>,
    Register: device_driver::FieldSet,
    Access: device_driver::ReadCapability + device_driver::WriteCapability,
{
    op.modify(f)
}

#[allow(dead_code)]
#[only_async]
async fn modify_internal<'a, Interface, Register, Access, R>(
    op: &mut device_driver::RegisterOperation<'a, Interface, u8, Register, Access>,
    f: impl FnOnce(&mut Register) -> R,
) -> Result<R, Interface::Error>
where
    Interface: RegisterInterface<AddressType = u8>,
    Register: device_driver::FieldSet,
    Access: device_driver::ReadCapability + device_driver::WriteCapability,
{
    op.modify_async(f).await
}