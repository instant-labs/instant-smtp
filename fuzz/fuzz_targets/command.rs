#![no_main]
use libfuzzer_sys::fuzz_target;
use instant_smtp::types::Command;

fuzz_target!(|data: &[u8]| {
    if let Ok((_, cmd)) = Command::from_bytes(data) {
        // Fuzzer created a valid SMTP command.
        // dbg!(&cmd);

        let cmd2 = {
            // Let's serialize the command into bytes ...
            let mut buf = Vec::new();
            cmd.serialize(&mut buf).unwrap();

            // ... parse it again ...
            let (rem, cmd2) = Command::from_bytes(&buf).unwrap();
            assert!(rem.is_empty());

            // dbg!(&cmd2);
            cmd2
        };

        // ... and verify that we got the same results.
        assert_eq!(cmd, cmd2);
    }
});
