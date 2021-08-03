use pgx::*;

pub const TEST_UUID_V4: UuidBytes = [
    0x12, 0x3e, 0x45, 0x67, 0xe8, 0x9b, 0x12, 0xd3, 0xa4, 0x56, 0x42, 0x66, 0x14, 0x17, 0x40, 0x00,
];

#[pg_extern]
fn accept_uuid(uuid: Uuid) -> Uuid {
    uuid
}

#[pg_extern]
fn return_uuid() -> Uuid {
    Uuid::from_bytes(TEST_UUID_V4)
}

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;
    use pgx::*;

    #[pg_test]
    fn test_accept_uuid() {
        let result = Spi::get_one::<bool>("SELECT accept_uuid('123e4567-e89b-12d3-a456-426614174000'::uuid) = '123e4567-e89b-12d3-a456-426614174000'::uuid;")
            .expect("failed to get SPI result");
        assert!(result)
    }

    #[pg_test]
    fn test_return_uuid() {
        let result = Spi::get_one::<bool>(
            "SELECT return_uuid() = '123e4567-e89b-12d3-a456-426614174000'::uuid;",
        )
        .expect("SPI result was null");
        assert!(result)
    }

    #[pg_test]
    fn test_parse_uuid_v4() {
        let uuid = Spi::get_one::<Uuid>("SELECT '123e4567-e89b-12d3-a456-426614174000'::uuid;")
            .expect("SPI result was null");
        assert_eq!(uuid, Uuid::from_bytes(super::TEST_UUID_V4))
    }
}
