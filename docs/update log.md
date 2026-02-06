# 改动记录

## 基本函数体方法和数据结构

* 为`MCI`添加`desc_list: FSdifIdmaDescList`字段，实现new方法

* 为`MCI`添加`cur_cmd: Option<MCICmdData>`字段，实现new方法。还没确定用Option还是指针

* 为`SDIFDevPIO`添加`rw_desc: *mut FSdifIdmaDesc`字段

* 修改`FSdifIDmaDescList::first_desc_dma`为usize类型；修改`MCIData::buf_dma`为usize类型，对应修改相关函数的定义

* 为MCIData添加buf_dma字段，用于记录dma传输的地址，相应在covert_command_info配置out_data的buf_dma

* mci/mod.rs添加dma相关函数

  * `pub fn set_idma_list(&mut self, desc: *mut FSdifIDmaDesc, desc_dma: u32, desc_num: u32) -> MCIResult`
  * `pub fn dma_transfer(&mut self, cmd_data: &mut MCICmdData) -> MCIResult`
  * `pub fn poll_wait_dma_end(&mut self, cmd_data: &mut MCICmdData) -> MCIResult`

* 剩余dma相关代码放到`mci_dma.rs`

  * `pub fn dump_dma_descriptor(&self, *desc_in_use*: u32)`
  * `pub(crate) fn setup_dma_descriptor(&mut self, *data*: &MCIData) -> MCIResult`
  * `pub(crate) fn dma_transfer_data(&mut self, *data*: &MCIData) -> MCIResult`


* commit 532156：

  1. 为`MCIHostDevice`添加type_id()，为了得到host的dev字段（添加`MCIHost::get_dev()`）。貌似可以用其他方法得到，目前先不修改
  2. 修改`SDIFDevPIO`的`do_init()`方法，无论是否启用dma均设置`dma_list`，后面还要修改
  3. 修改`MCI`的`restart`方法，可以根据传入的`MCIId`选择`restart`返回的类型
  4. 为`SdCard`添加`dma_rw_init()`方法，负责设备初始化完毕后启用dma传输之前的操作

* commit ef3d49:
  
  1. 调试发现是CMD17发送指令前`argument`配置有误，修改测试发现PIO读功能正常
  2. 切换为DMA模式发现解析SCR寄存器出现问题，导致后续报错不支持CMD6
  3. 在CPU读取DMA传输`buffer`之前利用`DSlice`获取该`buffer`引用，`DSlice`会自动调用`flush()`刷新缓存，测试发现仍然无法读出，暂存

* commit f461bd:

  1. 想到`desc_list`部分仍然使用的是虚拟地址，应该是这部分填写出了问题，进行针对性修改
  2. 由于`DVec`不提供`DerefMut`，考虑仍使用`DSlice`。与利用`DSlice`获取`DMA buffer`同理，先使用`Vec::from_raw_parts()`将`desc_list.first_desc`转换为数组，再用D`Slice`获取引用，得到`bus_addr`，即应该填入SD寄存器`MCIDescListAddrH`和`MCIDescListAddrL`的值。测试发现解析`scr`寄存器正常，不会再报错卡不支持CMD6
  3. 经过进一步调试，ACMD51（获取卡scr）为初始化中第一次需要接收数据的情况，后续第二次需要接收数据（CMD6或其他情况）时代码会卡死。调试发现因为`Vec::from_raw_parts()`会转移所有权，该`Vec`生命周期结束后会被自动释放导致后续访问`desc`无效。修改使用了`core::mem::ManullyDrop`来阻止编译器释放Vec的内存。至此DMA读成功，读出内容与dd命令相比较一致。
  4. 后续考虑使用更优雅的方法进行`flush()`

* 删除大量无用注释；发现invalidate函数确实有用，而且是必要的，但这样有的地方使用invalidate，有的地方使用DSlice，不太规范，后面再改；单块读写DMA/PIO模式都没问题，但DMA模式好像会额外拓展buffer长度，待修改；下一步解析多块读逻辑进行开发

## 初始化

* 为`SDIFDevPIO::new()`添加了`desc_num`字段，以便创建`SDIFDevPIO`实例时为`rw_desc`分配空间

## 其他

调整了部分代码格式，规范了部分函数的注释格式