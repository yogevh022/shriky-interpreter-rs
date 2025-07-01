pub enum ByteOperation {
    LoadName = 0x01,
    LoadGlobal = 0x02,
    LoadConst = 0x03,
    Add = 0x04,
    Return = 0x05,
    Call = 0x06,
    Pop = 0x07,
    StoreName = 0x08,
}
