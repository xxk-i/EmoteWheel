#![allow(warnings)]
use egui::{
    Align2, Color32, Context, FontData, FontDefinitions, FontFamily, FontId, FontTweak, Key,
    Modifiers, Pos2, Rect, RichText, ScrollArea, Slider, Stroke, TextureId, TextureOptions, Vec2,
    Widget, PointerButton,
};
use egui_d3d11::DirectX11App;
use faithe::{internal::alloc_console, pattern::Pattern};
use std::{
    intrinsics::transmute,
    sync::{Arc, Once},
};
use windows::{
    core::HRESULT,
    Win32::{
        Foundation::{HWND, LPARAM, LRESULT, WPARAM},
        Graphics::Dxgi::{Common::DXGI_FORMAT, IDXGISwapChain},
        UI::WindowsAndMessaging::{CallWindowProcW, SetWindowLongPtrA, GWLP_WNDPROC, WNDPROC},
    },
};

mod widget;
use widget::WheelWidget;

#[no_mangle]
extern "stdcall" fn DllMain(hinst: usize, reason: u32) -> i32 {
    if reason == 1 {
        std::thread::spawn(move || unsafe { main_thread(hinst) });
    }

    1
}

static mut APP: DirectX11App<i32> = DirectX11App::new();
static mut OLD_WND_PROC: Option<WNDPROC> = None;

type FnPresent = unsafe extern "stdcall" fn(IDXGISwapChain, u32, u32) -> HRESULT;
static mut O_PRESENT: Option<FnPresent> = None;

type FnResizeBuffers =
    unsafe extern "stdcall" fn(IDXGISwapChain, u32, u32, u32, DXGI_FORMAT, u32) -> HRESULT;
static mut O_RESIZE_BUFFERS: Option<FnResizeBuffers> = None;

unsafe extern "stdcall" fn hk_present(
    swap_chain: IDXGISwapChain,
    sync_interval: u32,
    flags: u32,
) -> HRESULT {
    static INIT: Once = Once::new();

    INIT.call_once(|| {
        APP.init_default(&swap_chain, ui);

        let desc = swap_chain.GetDesc().unwrap();
        if desc.OutputWindow.0 == -1 {
            panic!("Invalid window handle");
        }

        OLD_WND_PROC = Some(transmute(SetWindowLongPtrA(
            desc.OutputWindow,
            GWLP_WNDPROC,
            hk_wnd_proc as usize as _,
        )));
    });

    APP.present(&swap_chain);

    O_PRESENT.as_ref().unwrap()(swap_chain, sync_interval, flags)
}

unsafe extern "stdcall" fn hk_resize_buffers(
    swap_chain: IDXGISwapChain,
    buffer_count: u32,
    width: u32,
    height: u32,
    new_format: DXGI_FORMAT,
    swap_chain_flags: u32,
) -> HRESULT {
    eprintln!("Resizing buffers");

    APP.resize_buffers(&swap_chain, || {
        O_RESIZE_BUFFERS.as_ref().unwrap()(
            swap_chain.clone(),
            buffer_count,
            width,
            height,
            new_format,
            swap_chain_flags,
        )
    })
}

unsafe extern "stdcall" fn hk_wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    APP.wnd_proc(msg, wparam, lparam);

    CallWindowProcW(OLD_WND_PROC.unwrap(), hwnd, msg, wparam, lparam)
}

static mut FRAME: i32 = 0;
fn ui(ctx: &Context, i: &mut i32) {
    unsafe {
        static mut DISPLAY: bool = false;
        static mut WHEEL_WIDGET: Option<WheelWidget> = None;
        static ONCE: Once = Once::new();

        ONCE.call_once(|| {
            WHEEL_WIDGET = Some(WheelWidget::default());
        }); 

         // Single central panel that contains ONLY our EmoteWheel widget
        let mut panel = egui::CentralPanel::default();
        panel = panel.frame(egui::Frame { fill:egui::Color32::TRANSPARENT, ..Default::default()});

        if ctx.input().key_pressed(egui::Key::Space) {
            DISPLAY = !DISPLAY;
        }

        if DISPLAY {
            if ctx.input().pointer.button_clicked(PointerButton::Primary) {
                DISPLAY = false; 
            }

            else {
                panel.show(ctx,  |ui| {
                    match &mut WHEEL_WIDGET {
                        Some(x) => {
                            x.display(ui, ctx);
                        }
                        
                        None => {
                            panic!("What the fuck?");
                        }
                    }
                });
            }
        }
    }
}

unsafe fn main_thread(_hinst: usize) {
    let present = faithe::internal::find_pattern(
        "gameoverlayrenderer64.dll",
        Pattern::from_ida_style("48 89 6C 24 18 48 89 74 24 20 41 56 48 83 EC 20 41"),
    )
    .unwrap_or_else(|_| {
        faithe::internal::find_pattern(
            "dxgi.dll",
            Pattern::from_ida_style("48 89 5C 24 10 48 89 74 24 20 55 57 41 56"),
        )
        .unwrap()
    })
    .unwrap() as usize;

    eprintln!("Present: {:X}", present);

    let swap_buffers = faithe::internal::find_pattern(
        "gameoverlayrenderer64.dll",
        Pattern::from_ida_style(
            "48 89 5C 24 08 48 89 6C 24 10 48 89 74 24 18 57 41 56 41 57 48 83 EC 30 44",
        ),
    )
    .unwrap_or_else(|_| {
        faithe::internal::find_pattern(
            "dxgi.dll",
            Pattern::from_ida_style("48 8B C4 55 41 54 41 55 41 56 41 57 48 8D 68 B1 48 81 EC C0"),
        )
        .unwrap()
    })
    .unwrap() as usize;

    eprintln!("Buffers: {:X}", swap_buffers);

    sunshine::create_hook(
        sunshine::HookType::Compact,
        transmute::<_, FnPresent>(present),
        hk_present as FnPresent,
        &mut O_PRESENT,
    )
    .unwrap();

    sunshine::create_hook(
        sunshine::HookType::Compact,
        transmute::<_, FnResizeBuffers>(swap_buffers),
        hk_resize_buffers as FnResizeBuffers,
        &mut O_RESIZE_BUFFERS,
    )
    .unwrap();

    #[allow(clippy::empty_loop)]
    loop {}
}