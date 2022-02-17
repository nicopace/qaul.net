//Generated by the protocol buffer compiler. DO NOT EDIT!
// source: connections/ble/manager/ble.proto

package qaul.sys.ble;

@kotlin.jvm.JvmSynthetic
public inline fun bleDirectSend(block: qaul.sys.ble.BleDirectSendKt.Dsl.() -> kotlin.Unit): qaul.sys.ble.BleOuterClass.BleDirectSend =
  qaul.sys.ble.BleDirectSendKt.Dsl._create(qaul.sys.ble.BleOuterClass.BleDirectSend.newBuilder()).apply { block() }._build()
public object BleDirectSendKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: qaul.sys.ble.BleOuterClass.BleDirectSend.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: qaul.sys.ble.BleOuterClass.BleDirectSend.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): qaul.sys.ble.BleOuterClass.BleDirectSend = _builder.build()

    /**
     * <pre>
     * message id (as a reference for the result message)
     * </pre>
     *
     * <code>bytes id = 1;</code>
     */
    public var id: com.google.protobuf.ByteString
      @JvmName("getId")
      get() = _builder.getId()
      @JvmName("setId")
      set(value) {
        _builder.setId(value)
      }
    /**
     * <pre>
     * message id (as a reference for the result message)
     * </pre>
     *
     * <code>bytes id = 1;</code>
     */
    public fun clearId() {
      _builder.clearId()
    }

    /**
     * <pre>
     * bluetooth address of the device to send it to
     * </pre>
     *
     * <code>bytes to = 2;</code>
     */
    public var to: com.google.protobuf.ByteString
      @JvmName("getTo")
      get() = _builder.getTo()
      @JvmName("setTo")
      set(value) {
        _builder.setTo(value)
      }
    /**
     * <pre>
     * bluetooth address of the device to send it to
     * </pre>
     *
     * <code>bytes to = 2;</code>
     */
    public fun clearTo() {
      _builder.clearTo()
    }

    /**
     * <pre>
     * sending mode
     * </pre>
     *
     * <code>.qaul.sys.ble.BleMode mode = 3;</code>
     */
    public var mode: qaul.sys.ble.BleOuterClass.BleMode
      @JvmName("getMode")
      get() = _builder.getMode()
      @JvmName("setMode")
      set(value) {
        _builder.setMode(value)
      }
    /**
     * <pre>
     * sending mode
     * </pre>
     *
     * <code>.qaul.sys.ble.BleMode mode = 3;</code>
     */
    public fun clearMode() {
      _builder.clearMode()
    }

    /**
     * <pre>
     * data to be sent
     * </pre>
     *
     * <code>bytes data = 4;</code>
     */
    public var data: com.google.protobuf.ByteString
      @JvmName("getData")
      get() = _builder.getData()
      @JvmName("setData")
      set(value) {
        _builder.setData(value)
      }
    /**
     * <pre>
     * data to be sent
     * </pre>
     *
     * <code>bytes data = 4;</code>
     */
    public fun clearData() {
      _builder.clearData()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun qaul.sys.ble.BleOuterClass.BleDirectSend.copy(block: qaul.sys.ble.BleDirectSendKt.Dsl.() -> kotlin.Unit): qaul.sys.ble.BleOuterClass.BleDirectSend =
  qaul.sys.ble.BleDirectSendKt.Dsl._create(this.toBuilder()).apply { block() }._build()
