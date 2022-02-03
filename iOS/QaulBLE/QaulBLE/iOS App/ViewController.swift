//
//  ViewController.swift
//  QaulBLE
//
//  Created by BAPS on 12/01/22.
//

import UIKit

var navigationcontroller = UINavigationController()

class ViewController: UIViewController {

    //-----------------------------------------------------------------
    //                        MARK: - Outlet -
    //-----------------------------------------------------------------
    
    @IBOutlet weak var txtQaulBLE: UITextField!
    @IBOutlet weak var txtMessage: UITextView!
    @IBOutlet weak var lblMessagePlaceholader: UILabel!
    @IBOutlet weak var heightOfTxtMessage: NSLayoutConstraint!

    // -----------------------------------------------------------------
    //                        MARK: - Property -
    // -----------------------------------------------------------------
    private var value = "iOSQaulBLE"
    private var qaulId: String = ""
    private let maxHeightOfTxtMessage: CGFloat = 1000

    //-----------------------------------------------------------------
    //                       MARK: - View Life Cycle -
    //-----------------------------------------------------------------
    
    override func viewDidLoad() {
        super.viewDidLoad()
        
        navigationcontroller = self.navigationController ?? UINavigationController()
        // Do any additional setup after loading the view.
        
        NotificationCenter.default.removeObserver(self, name: .GetscanDevice, object: nil)
        NotificationCenter.default.addObserver(self, selector: #selector(SetScanDevice(_:)), name: .GetscanDevice, object: nil)
        
    }
    
    //-----------------------------------------------------------------
    //                    MARK: - Button Action -
    //-----------------------------------------------------------------
    @IBAction func btnInfoRequest(sender: UIButton) {
        
        var info = Qaul_Sys_Ble_BleInfoRequest.init()

        var initobj = Qaul_Sys_Ble_Ble.init()
        initobj.infoRequest = info
        initobj.message = .infoRequest(info)
        print(initobj.message)

        var setbleReq = Qaul_Sys_Ble_Ble.init()
        setbleReq.message = .infoRequest(Qaul_Sys_Ble_BleInfoRequest.init())
        
        bleWrapperClass.receiveRequest(bleReq: initobj, SetdataforbleReq: setbleReq) { qaul_Sys_Ble_Ble in
            print("qaul_Sys_Ble_Ble:- \(qaul_Sys_Ble_Ble)")
            if qaul_Sys_Ble_Ble.infoResponse != nil {
                let strmessage = "Device info recived from : \(qaul_Sys_Ble_Ble.infoResponse.device.name)"
                DispatchQueue.main.async {
                    self.view.makeToast(strmessage)
                }
            }
        }
    }
    
    @IBAction func btnAdvertizing(sender: UIButton) {
        
//        blePeripheral.startAdvertising(serviceID: kTRANSFER_SERVICE_UUID, name: self.value)
        sendStartRequest()
    }
    
    @IBAction func btnStopAdvertizing(sender: UIButton) {
        
//        blePeripheral.startAdvertising(serviceID: kTRANSFER_SERVICE_UUID, name: self.value)
        sendStopRequest()
    }
    
    @IBAction func btnSendMessage(sender: UIButton) {
        
       
    }
    
    //-----------------------------------------------------------------
    //                    MARK: - Functions -
    //-----------------------------------------------------------------
    
    @objc func SetScanDevice(_ notification: NSNotification) {
        guard let strQaulID = notification.object as? BLEScanDevice else { return }
       
        self.txtQaulBLE.text = strQaulID.strqaulId
        
    }
    
    /**
     * For Sending BleStartRequest to BLEModule
     * Have to pass qaul_id and advertise_mode as parameter
     */
    private func sendStartRequest() {
    
        var startRequest = Qaul_Sys_Ble_BleStartRequest.init()
    
        startRequest.qaulID = (appendtextiOSdevice + UIDevice.modelName).data(using: .utf8)!
        startRequest.mode = Qaul_Sys_Ble_BleMode.lowLatency //BleOuterClass.BleMode.low_latency
    
        var bleReq = Qaul_Sys_Ble_Ble.init()
        bleReq.startRequest = startRequest
//        bleReq.message = .startRequest(Qaul_Sys_Ble_BleStartRequest.init())
    
        var setbleReq = Qaul_Sys_Ble_Ble.init()
        setbleReq.message = .startRequest(Qaul_Sys_Ble_BleStartRequest.init())
        
        bleWrapperClass.receiveRequest(bleReq: bleReq, SetdataforbleReq: setbleReq) { qaul_Sys_Ble_Ble in
            print("qaul_Sys_Ble_Ble:- \(qaul_Sys_Ble_Ble)")
            if qaul_Sys_Ble_Ble.startResult != nil {
                let strmessage = qaul_Sys_Ble_Ble.startResult.errorMessage
                DispatchQueue.main.async {
                    self.view.makeToast(strmessage)
                }
            }
        }
    }
    
    /**
     * For Sending BleStopRequest to BLEModule. It Is Used To Stop Service.
     */
    private func sendStopRequest() {
      
        var stopRequest = Qaul_Sys_Ble_BleStopRequest.init()
     
        var bleReq = Qaul_Sys_Ble_Ble.init()
        bleReq.message = .stopRequest(Qaul_Sys_Ble_BleStopRequest.init())
        bleReq.stopRequest = stopRequest
        
        var setbleReq = Qaul_Sys_Ble_Ble.init()
        setbleReq.message = .stopRequest( Qaul_Sys_Ble_BleStopRequest.init())
        
        bleWrapperClass.receiveRequest(bleReq: bleReq, SetdataforbleReq: setbleReq) { qaul_Sys_Ble_Ble in
            print("qaul_Sys_Ble_Ble:- \(qaul_Sys_Ble_Ble)")
            if qaul_Sys_Ble_Ble.stopResult != nil {
                let strmessage = qaul_Sys_Ble_Ble.stopResult.errorMessage
                DispatchQueue.main.async {
                    self.view.makeToast(strmessage)
                }
            }
        }
    }
    
}



//public extension String {
//    
//    var bytes: Array<UInt8> {
//        return data(using: String.Encoding.utf8, allowLossyConversion: true)?.bytes ?? Array(utf8)
//    }
//    
//}
// ---------------------------------------------------------------------------
//                          MARK: - UITextViewDelegate -
// ---------------------------------------------------------------------------
extension ViewController: UITextViewDelegate {

func textView(_ textView: UITextView, shouldChangeTextIn range: NSRange, replacementText text: String) -> Bool {
    
    let updatedText = (textView.text ?? "").count + text.count - range.length
    
    switch textView {
        case self.txtMessage:
            
            if updatedText > 0 {
                self.lblMessagePlaceholader.isHidden = true
                
            } else {
                self.lblMessagePlaceholader.isHidden = false
            }
            
            if self.txtMessage.contentSize.height >= self.maxHeightOfTxtMessage {
                self.txtMessage.isScrollEnabled = true
                    //                    self.heightOfTxtMessage.constant = 37

            } else {
                self.txtMessage.isScrollEnabled = false
                self.heightOfTxtMessage.constant = self.maxHeightOfTxtMessage
            }
            return updatedText <= 1500
            
        default:
            break
    }
    return true
}
}
