pub extern crate imgui_sys as sys;
use bitflags::bitflags;
use imgui::{Condition, Direction, Id};
use sys::ImGuiID;

bitflags! {
    pub struct DockNodeFlags: u32 {
        const NONE = sys::ImGuiDockNodeFlags_None;
        const KEEP_ALIVE_ONLY = sys::ImGuiDockNodeFlags_KeepAliveOnly;
        const NO_DOCKING_OVER_CENTRAL_NODE = sys::ImGuiDockNodeFlags_NoDockingOverCentralNode;
        const PASS_THRU_CENTRAL_NODE = sys::ImGuiDockNodeFlags_PassthruCentralNode;
        const NO_SPLIT = sys::ImGuiDockNodeFlags_NoDockingSplit;
        const NO_RESIZE = sys::ImGuiDockNodeFlags_NoResize;
        const AUTO_HIDE_TAB_BAR = sys::ImGuiDockNodeFlags_AutoHideTabBar;
        const NO_UNDOCKING = sys::ImGuiDockNodeFlags_NoUndocking;

        const DOCK_SPACE = sys::ImGuiDockNodeFlags_DockSpace as u32;
        const CENTRAL_NODE = sys::ImGuiDockNodeFlags_CentralNode as u32;
        const NO_TAB_BAR = sys::ImGuiDockNodeFlags_NoTabBar as u32;
        const HIDDEN_TAB_BAR = sys::ImGuiDockNodeFlags_HiddenTabBar as u32;
        const NO_WINDOW_MENU_BUTTON = sys::ImGuiDockNodeFlags_NoWindowMenuButton as u32;
        const NO_CLOSE_BUTTON = sys::ImGuiDockNodeFlags_NoCloseButton as u32;
        const NO_RESIZE_X = sys::ImGuiDockNodeFlags_NoResizeX as u32;
        const NO_RESIZE_Y = sys::ImGuiDockNodeFlags_NoResizeY as u32;
        const DOCKED_WINDOWS_IN_FOCUS_ROUTE = sys::ImGuiDockNodeFlags_DockedWindowsInFocusRoute as u32;
        const NO_DOCKING_SPLIT_OTHER = sys::ImGuiDockNodeFlags_NoDockingSplitOther as u32;
        const NO_DOCKING_OVER_ME = sys::ImGuiDockNodeFlags_NoDockingOverMe as u32;
        const NO_DOCKING_OVER_OTHER = sys::ImGuiDockNodeFlags_NoDockingOverOther as u32;
        const NO_DOCKING_OVER_EMPTY = sys::ImGuiDockNodeFlags_NoDockingOverEmpty as u32;
        const NO_DOCKING = sys::ImGuiDockNodeFlags_NoDocking as u32;
    }
}

pub struct Dock;

impl Dock {
    pub fn dock_space(&self, id: Id, size: [f32; 2]) {
        unsafe {
            let id: ImGuiID = std::mem::transmute(id);
            sys::igDockSpace(id, size.into(), 0, std::ptr::null());
        }
    }

    // TODO: finish these APIs.
    //pub fn dock_space_over_viewport() -> ImGuiID {
    //}
    //pub fn set_next_window_class() -> ImGuiID {
    //}

    pub fn set_next_window_dock_id(&self, id: Id, cond: Condition) {
        unsafe {
            let id: ImGuiID = std::mem::transmute(id);
            sys::igSetNextWindowDockID(id, cond as i32);
        }
    }

    pub fn get_window_dock_id(&self) -> Id {
        unsafe {
            let id = sys::igGetWindowDockID();
            std::mem::transmute(id)
        }
    }

    pub fn is_window_docked(&self) -> bool {
        unsafe { sys::igIsWindowDocked() }
    }

    pub fn dock_builder_has_node(&self, id: Id) -> bool {
        unsafe {
            let id: ImGuiID = std::mem::transmute(id);
            sys::igDockBuilderGetNode(id) != std::ptr::null_mut()
        }
    }

    pub fn dock_builder_remove_node(&self, id: Id) {
        unsafe {
            let id: ImGuiID = std::mem::transmute(id);
            sys::igDockBuilderRemoveNode(id);
        }
    }

    pub fn dock_builder_add_node(&self, id: Id, flags: DockNodeFlags) {
        unsafe {
            let id: ImGuiID = std::mem::transmute(id);
            sys::igDockBuilderAddNode(id, flags.bits() as i32);
        }
    }

    pub fn dock_builder_set_node_size(&self, id: Id, size: [f32; 2]) {
        unsafe {
            let id: ImGuiID = std::mem::transmute(id);
            sys::igDockBuilderSetNodeSize(id, size.into());
        }
    }

    pub fn dock_builder_split_node(
        &self,
        id: Id,
        split_dir: Direction,
        split_ratio: f32,
    ) -> (Id, Id) {
        unsafe {
            let id: ImGuiID = std::mem::transmute(id);
            let mut opposite: sys::ImGuiID = 0;
            let id1 = sys::igDockBuilderSplitNode(
                id,
                split_dir as i32,
                split_ratio,
                std::ptr::null_mut(),
                &mut opposite as *mut u32,
            );
            (std::mem::transmute(id1), std::mem::transmute(opposite))
        }
    }

    pub fn dock_builder_dock_window(&self, window_name: &str, id: Id) {
        unsafe {
            let window_name = std::ffi::CString::new(window_name).expect("CString::new");
            let id: ImGuiID = std::mem::transmute(id);
            sys::igDockBuilderDockWindow(window_name.as_ptr(), id);
        }
    }

    pub fn dock_builder_finish(&self, id: Id) {
        unsafe {
            let id: ImGuiID = std::mem::transmute(id);
            sys::igDockBuilderFinish(id);
        }
    }
}
