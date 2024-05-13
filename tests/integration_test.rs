use std::fs;

#[test]
fn test_json() {
    let raw_transaction_hex = "0100000001d4f92ee4e8ad1a99c4a76f562ddc2345191a76d6c0db7e766859e01d3bedfe0a000000006b483045022100cab4dbf51074f2ed4255824fe7a4723217415fbe4209561e031ca54400f5243c022034960b9f49685952ce88971288f057e1394ea591982d5df9bc6ede95be35f4e3012103a3deb6df91d41e4d062b429004a31f9e070182b8e548c43d165e863a08119df9ffffffff0158020000000000001976a9142b1da6ec2d055aa03cad386841a4b4dc62acd5b688ac00000000";
    let json = transactiondecoder::run(raw_transaction_hex.to_string()).unwrap();
    let expected = fs::read_to_string("tests/test_transaction.json").unwrap();
    assert_eq!(expected, json);
}