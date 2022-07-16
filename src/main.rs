use floui::{enums::*, prelude::*, widgets::*};
use objc::declare::ClassDecl;
use objc::runtime::{Object, Sel, BOOL, YES};
use objc::{class, msg_send, sel, sel_impl};
use std::cell::RefCell;
use std::os::raw::c_char;
use std::ptr;
use std::rc::Rc;

#[repr(C)]
struct Frame(f64, f64, f64, f64);

extern "C" {
    fn UIApplicationMain(
        argc: i32,
        argv: *mut *mut c_char,
        principalClass: *mut Object,
        delegateName: *mut Object,
    ) -> i32;
}

extern "C" fn did_finish_launching_with_options(
    obj: &mut Object,
    _: Sel,
    _: *mut Object,
    _: *mut Object,
) -> BOOL {
    unsafe {
        let frame: *mut Object = msg_send![class!(UIScreen), mainScreen];
        let frame: Frame = msg_send![frame, bounds];
        let win: *mut Object = msg_send![class!(UIWindow), alloc];
        let win: *mut Object = msg_send![win, initWithFrame: frame];
        let vc: *mut Object = msg_send![class!(ViewController), new];
        let _: () = msg_send![win, setRootViewController: vc];
        let _: () = msg_send![win, makeKeyAndVisible];
        let white: *mut Object = msg_send![class!(UIColor), whiteColor];
        let _: () = msg_send![win, setBackgroundColor: white];
        obj.set_ivar("window", win as usize);
    }
    YES
}

extern "C" fn did_load(obj: &mut Object, _: Sel) {
    let _: () = unsafe { msg_send![super(obj, class!(UIViewController)), viewDidLoad] };
    let vc = unsafe {
        ViewController::new(
            obj as *mut Object as _,
            "Counter\0".as_ptr() as _,
            ptr::null_mut(),
        )
    };
    mygui(&vc);
}

pub fn prep() {
    let ui_responder_cls = class!(UIResponder);
    let mut app_delegate_cls = ClassDecl::new("AppDelegate", ui_responder_cls).unwrap();

    unsafe {
        app_delegate_cls.add_method(
            sel!(application:didFinishLaunchingWithOptions:),
            did_finish_launching_with_options
                as extern "C" fn(&mut Object, Sel, *mut Object, *mut Object) -> BOOL,
        );

        app_delegate_cls.add_ivar::<usize>("window");

        app_delegate_cls.register();
    }

    let ui_view_controller_cls = class!(UIViewController);
    let mut view_controller_cls = ClassDecl::new("ViewController", ui_view_controller_cls).unwrap();

    unsafe {
        view_controller_cls.add_method(
            sel!(viewDidLoad),
            did_load as extern "C" fn(&mut Object, Sel),
        );

        view_controller_cls.register();
    }
}

fn mygui(vc: &ViewController) -> MainView {
    let count = Rc::from(RefCell::from(0));
    MainView::new(
        &vc,
        &[
            &Button::new("Increment").foreground(Color::Blue).action({
                let count = count.clone();
                move |_| {
                    log("Increment clicked");
                    let mut c = count.borrow_mut();
                    *c += 1;
                    let t: Text = from_id("mytext").unwrap();
                    t.text(&format!("{}", c));
                }
            }),
            &Text::new("0").id("mytext").center().bold(),
            &Button::new("Decrement")
                .foreground(Color::Red)
                .action(move |_| {
                    log("Decrement clicked");
                    let mut c = count.borrow_mut();
                    *c -= 1;
                    let t: Text = from_id("mytext").unwrap();
                    t.text(&format!("{}", c));
                }),
        ],
    )
}

fn main() {
    unsafe {
        prep();
        let name: *mut Object =
            msg_send![class!(NSString), stringWithUTF8String:"AppDelegate\0".as_ptr()];
        UIApplicationMain(0, ptr::null_mut(), ptr::null_mut(), name);
    }
}
