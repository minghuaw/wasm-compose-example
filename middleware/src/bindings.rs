// Generated by `wit-bindgen` 0.21.0. DO NOT EDIT!
// Options used:
pub mod example {
    pub mod service {
        #[allow(clippy::all)]
        pub mod logging {
            #[used]
            #[doc(hidden)]
            #[cfg(target_arch = "wasm32")]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;

            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Logger {
                handle: _rt::Resource<Logger>,
            }

            impl Logger {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }

                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }

                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }

            unsafe impl _rt::WasmResource for Logger {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();

                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "example:service/logging@0.1.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]logger"]
                            fn drop(_: u32);
                        }

                        drop(_handle);
                    }
                }
            }

            impl Logger {
                #[allow(unused_unsafe, clippy::all)]
                pub fn log(&self, message: &str) {
                    unsafe {
                        let vec0 = message;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();

                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "example:service/logging@0.1.0")]
                        extern "C" {
                            #[link_name = "[method]logger.log"]
                            fn wit_import(_: i32, _: *mut u8, _: usize);
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0.cast_mut(), len0);
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn get_logger() -> Logger {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "example:service/logging@0.1.0")]
                    extern "C" {
                        #[link_name = "get-logger"]
                        fn wit_import() -> i32;
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    Logger::from_handle(ret as u32)
                }
            }
        }

        #[allow(clippy::all)]
        pub mod types {
            #[used]
            #[doc(hidden)]
            #[cfg(target_arch = "wasm32")]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;

            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Request {
                handle: _rt::Resource<Request>,
            }

            impl Request {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }

                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }

                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }

            unsafe impl _rt::WasmResource for Request {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();

                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "example:service/types@0.1.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]request"]
                            fn drop(_: u32);
                        }

                        drop(_handle);
                    }
                }
            }

            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Response {
                handle: _rt::Resource<Response>,
            }

            impl Response {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }

                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }

                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }

            unsafe impl _rt::WasmResource for Response {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();

                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "example:service/types@0.1.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]response"]
                            fn drop(_: u32);
                        }

                        drop(_handle);
                    }
                }
            }

            #[repr(u8)]
            #[derive(Clone, Copy, Eq, PartialEq)]
            pub enum Error {
                BadRequest,
            }
            impl Error {
                pub fn name(&self) -> &'static str {
                    match self {
                        Error::BadRequest => "bad-request",
                    }
                }
                pub fn message(&self) -> &'static str {
                    match self {
                        Error::BadRequest => "",
                    }
                }
            }
            impl ::core::fmt::Debug for Error {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("Error")
                        .field("code", &(*self as i32))
                        .field("name", &self.name())
                        .field("message", &self.message())
                        .finish()
                }
            }
            impl ::core::fmt::Display for Error {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    write!(f, "{} (error {})", self.name(), *self as i32)
                }
            }

            impl std::error::Error for Error {}

            impl Error {
                pub(crate) unsafe fn _lift(val: u8) -> Error {
                    if !cfg!(debug_assertions) {
                        return ::core::mem::transmute(val);
                    }

                    match val {
                        0 => Error::BadRequest,

                        _ => panic!("invalid enum discriminant"),
                    }
                }
            }

            impl Request {
                #[allow(unused_unsafe, clippy::all)]
                pub fn new(headers: &[(_rt::Vec<u8>, _rt::Vec<u8>)], body: &[u8]) -> Self {
                    unsafe {
                        let vec3 = headers;
                        let len3 = vec3.len();
                        let layout3 =
                            _rt::alloc::Layout::from_size_align_unchecked(vec3.len() * 16, 4);
                        let result3 = if layout3.size() != 0 {
                            let ptr = _rt::alloc::alloc(layout3).cast::<u8>();
                            if ptr.is_null() {
                                _rt::alloc::handle_alloc_error(layout3);
                            }
                            ptr
                        } else {
                            {
                                ::core::ptr::null_mut()
                            }
                        };
                        for (i, e) in vec3.into_iter().enumerate() {
                            let base = result3.add(i * 16);
                            {
                                let (t0_0, t0_1) = e;
                                let vec1 = t0_0;
                                let ptr1 = vec1.as_ptr().cast::<u8>();
                                let len1 = vec1.len();
                                *base.add(4).cast::<usize>() = len1;
                                *base.add(0).cast::<*mut u8>() = ptr1.cast_mut();
                                let vec2 = t0_1;
                                let ptr2 = vec2.as_ptr().cast::<u8>();
                                let len2 = vec2.len();
                                *base.add(12).cast::<usize>() = len2;
                                *base.add(8).cast::<*mut u8>() = ptr2.cast_mut();
                            }
                        }
                        let vec4 = body;
                        let ptr4 = vec4.as_ptr().cast::<u8>();
                        let len4 = vec4.len();

                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "example:service/types@0.1.0")]
                        extern "C" {
                            #[link_name = "[constructor]request"]
                            fn wit_import(_: *mut u8, _: usize, _: *mut u8, _: usize) -> i32;
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: *mut u8, _: usize, _: *mut u8, _: usize) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(result3, len3, ptr4.cast_mut(), len4);
                        if layout3.size() != 0 {
                            _rt::alloc::dealloc(result3.cast(), layout3);
                        }
                        Request::from_handle(ret as u32)
                    }
                }
            }
            impl Request {
                #[allow(unused_unsafe, clippy::all)]
                pub fn headers(&self) -> _rt::Vec<(_rt::Vec<u8>, _rt::Vec<u8>)> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "example:service/types@0.1.0")]
                        extern "C" {
                            #[link_name = "[method]request.headers"]
                            fn wit_import(_: i32, _: *mut u8);
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = *ptr0.add(0).cast::<*mut u8>();
                        let l2 = *ptr0.add(4).cast::<usize>();
                        let base9 = l1;
                        let len9 = l2;
                        let mut result9 = _rt::Vec::with_capacity(len9);
                        for i in 0..len9 {
                            let base = base9.add(i * 16);
                            let e9 = {
                                let l3 = *base.add(0).cast::<*mut u8>();
                                let l4 = *base.add(4).cast::<usize>();
                                let len5 = l4;
                                let l6 = *base.add(8).cast::<*mut u8>();
                                let l7 = *base.add(12).cast::<usize>();
                                let len8 = l7;

                                (
                                    _rt::Vec::from_raw_parts(l3.cast(), len5, len5),
                                    _rt::Vec::from_raw_parts(l6.cast(), len8, len8),
                                )
                            };
                            result9.push(e9);
                        }
                        _rt::cabi_dealloc(base9, len9 * 16, 4);
                        result9
                    }
                }
            }
            impl Request {
                #[allow(unused_unsafe, clippy::all)]
                pub fn body(&self) -> _rt::Vec<u8> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "example:service/types@0.1.0")]
                        extern "C" {
                            #[link_name = "[method]request.body"]
                            fn wit_import(_: i32, _: *mut u8);
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = *ptr0.add(0).cast::<*mut u8>();
                        let l2 = *ptr0.add(4).cast::<usize>();
                        let len3 = l2;
                        _rt::Vec::from_raw_parts(l1.cast(), len3, len3)
                    }
                }
            }
            impl Response {
                #[allow(unused_unsafe, clippy::all)]
                pub fn new(headers: &[(_rt::Vec<u8>, _rt::Vec<u8>)], body: &[u8]) -> Self {
                    unsafe {
                        let vec3 = headers;
                        let len3 = vec3.len();
                        let layout3 =
                            _rt::alloc::Layout::from_size_align_unchecked(vec3.len() * 16, 4);
                        let result3 = if layout3.size() != 0 {
                            let ptr = _rt::alloc::alloc(layout3).cast::<u8>();
                            if ptr.is_null() {
                                _rt::alloc::handle_alloc_error(layout3);
                            }
                            ptr
                        } else {
                            {
                                ::core::ptr::null_mut()
                            }
                        };
                        for (i, e) in vec3.into_iter().enumerate() {
                            let base = result3.add(i * 16);
                            {
                                let (t0_0, t0_1) = e;
                                let vec1 = t0_0;
                                let ptr1 = vec1.as_ptr().cast::<u8>();
                                let len1 = vec1.len();
                                *base.add(4).cast::<usize>() = len1;
                                *base.add(0).cast::<*mut u8>() = ptr1.cast_mut();
                                let vec2 = t0_1;
                                let ptr2 = vec2.as_ptr().cast::<u8>();
                                let len2 = vec2.len();
                                *base.add(12).cast::<usize>() = len2;
                                *base.add(8).cast::<*mut u8>() = ptr2.cast_mut();
                            }
                        }
                        let vec4 = body;
                        let ptr4 = vec4.as_ptr().cast::<u8>();
                        let len4 = vec4.len();

                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "example:service/types@0.1.0")]
                        extern "C" {
                            #[link_name = "[constructor]response"]
                            fn wit_import(_: *mut u8, _: usize, _: *mut u8, _: usize) -> i32;
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: *mut u8, _: usize, _: *mut u8, _: usize) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(result3, len3, ptr4.cast_mut(), len4);
                        if layout3.size() != 0 {
                            _rt::alloc::dealloc(result3.cast(), layout3);
                        }
                        Response::from_handle(ret as u32)
                    }
                }
            }
            impl Response {
                #[allow(unused_unsafe, clippy::all)]
                pub fn headers(&self) -> _rt::Vec<(_rt::Vec<u8>, _rt::Vec<u8>)> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "example:service/types@0.1.0")]
                        extern "C" {
                            #[link_name = "[method]response.headers"]
                            fn wit_import(_: i32, _: *mut u8);
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = *ptr0.add(0).cast::<*mut u8>();
                        let l2 = *ptr0.add(4).cast::<usize>();
                        let base9 = l1;
                        let len9 = l2;
                        let mut result9 = _rt::Vec::with_capacity(len9);
                        for i in 0..len9 {
                            let base = base9.add(i * 16);
                            let e9 = {
                                let l3 = *base.add(0).cast::<*mut u8>();
                                let l4 = *base.add(4).cast::<usize>();
                                let len5 = l4;
                                let l6 = *base.add(8).cast::<*mut u8>();
                                let l7 = *base.add(12).cast::<usize>();
                                let len8 = l7;

                                (
                                    _rt::Vec::from_raw_parts(l3.cast(), len5, len5),
                                    _rt::Vec::from_raw_parts(l6.cast(), len8, len8),
                                )
                            };
                            result9.push(e9);
                        }
                        _rt::cabi_dealloc(base9, len9 * 16, 4);
                        result9
                    }
                }
            }
            impl Response {
                #[allow(unused_unsafe, clippy::all)]
                pub fn body(&self) -> _rt::Vec<u8> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "example:service/types@0.1.0")]
                        extern "C" {
                            #[link_name = "[method]response.body"]
                            fn wit_import(_: i32, _: *mut u8);
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = *ptr0.add(0).cast::<*mut u8>();
                        let l2 = *ptr0.add(4).cast::<usize>();
                        let len3 = l2;
                        _rt::Vec::from_raw_parts(l1.cast(), len3, len3)
                    }
                }
            }
        }

        #[allow(clippy::all)]
        pub mod handler {
            #[used]
            #[doc(hidden)]
            #[cfg(target_arch = "wasm32")]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type Request = super::super::super::example::service::types::Request;
            pub type Response = super::super::super::example::service::types::Response;
            pub type Error = super::super::super::example::service::types::Error;
            #[allow(unused_unsafe, clippy::all)]
            pub fn execute(req: Request) -> Result<Response, Error> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "example:service/handler@0.1.0")]
                    extern "C" {
                        #[link_name = "execute"]
                        fn wit_import(_: i32, _: *mut u8);
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import((&req).take_handle() as i32, ptr0);
                    let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                    match l1 {
                        0 => {
                            let e = {
                                let l2 = *ptr0.add(4).cast::<i32>();

                                super::super::super::example::service::types::Response::from_handle(
                                    l2 as u32,
                                )
                            };
                            Ok(e)
                        }
                        1 => {
                            let e = {
                                let l3 = i32::from(*ptr0.add(4).cast::<u8>());

                                super::super::super::example::service::types::Error::_lift(l3 as u8)
                            };
                            Err(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
        }
    }
}
pub mod exports {
    pub mod example {
        pub mod service {
            #[allow(clippy::all)]
            pub mod handler {
                #[used]
                #[doc(hidden)]
                #[cfg(target_arch = "wasm32")]
                static __FORCE_SECTION_REF: fn() =
                    super::super::super::super::__link_custom_section_describing_imports;
                pub type Request = super::super::super::super::example::service::types::Request;
                pub type Response = super::super::super::super::example::service::types::Response;
                pub type Error = super::super::super::super::example::service::types::Error;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_execute_cabi<T: Guest>(arg0: i32) -> *mut u8 {
                    let result0 = T::execute(
                        super::super::super::super::example::service::types::Request::from_handle(
                            arg0 as u32,
                        ),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        Ok(e) => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                            *ptr1.add(4).cast::<i32>() = (e).take_handle() as i32;
                        }
                        Err(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            *ptr1.add(4).cast::<u8>() = (e.clone() as i32) as u8;
                        }
                    };
                    ptr1
                }
                pub trait Guest {
                    fn execute(req: Request) -> Result<Response, Error>;
                }
                #[doc(hidden)]

                macro_rules! __export_example_service_handler_0_1_0_cabi{
        ($ty:ident with_types_in $($path_to_types:tt)*) => (const _: () = {

          #[export_name = "example:service/handler@0.1.0#execute"]
          unsafe extern "C" fn export_execute(arg0: i32,) -> *mut u8 {
            $($path_to_types)*::_export_execute_cabi::<$ty>(arg0)
          }
        };);
      }
                #[doc(hidden)]
                pub(crate) use __export_example_service_handler_0_1_0_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 8]);
                static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 8]);
            }
        }
    }
}
mod _rt {

    use core::fmt;
    use core::marker;
    use core::sync::atomic::{AtomicU32, Ordering::Relaxed};

    /// A type which represents a component model resource, either imported or
    /// exported into this component.
    ///
    /// This is a low-level wrapper which handles the lifetime of the resource
    /// (namely this has a destructor). The `T` provided defines the component model
    /// intrinsics that this wrapper uses.
    ///
    /// One of the chief purposes of this type is to provide `Deref` implementations
    /// to access the underlying data when it is owned.
    ///
    /// This type is primarily used in generated code for exported and imported
    /// resources.
    #[repr(transparent)]
    pub struct Resource<T: WasmResource> {
        // NB: This would ideally be `u32` but it is not. The fact that this has
        // interior mutability is not exposed in the API of this type except for the
        // `take_handle` method which is supposed to in theory be private.
        //
        // This represents, almost all the time, a valid handle value. When it's
        // invalid it's stored as `u32::MAX`.
        handle: AtomicU32,
        _marker: marker::PhantomData<T>,
    }

    /// A trait which all wasm resources implement, namely providing the ability to
    /// drop a resource.
    ///
    /// This generally is implemented by generated code, not user-facing code.
    pub unsafe trait WasmResource {
        /// Invokes the `[resource-drop]...` intrinsic.
        unsafe fn drop(handle: u32);
    }

    impl<T: WasmResource> Resource<T> {
        #[doc(hidden)]
        pub unsafe fn from_handle(handle: u32) -> Self {
            debug_assert!(handle != u32::MAX);
            Self {
                handle: AtomicU32::new(handle),
                _marker: marker::PhantomData,
            }
        }

        /// Takes ownership of the handle owned by `resource`.
        ///
        /// Note that this ideally would be `into_handle` taking `Resource<T>` by
        /// ownership. The code generator does not enable that in all situations,
        /// unfortunately, so this is provided instead.
        ///
        /// Also note that `take_handle` is in theory only ever called on values
        /// owned by a generated function. For example a generated function might
        /// take `Resource<T>` as an argument but then call `take_handle` on a
        /// reference to that argument. In that sense the dynamic nature of
        /// `take_handle` should only be exposed internally to generated code, not
        /// to user code.
        #[doc(hidden)]
        pub fn take_handle(resource: &Resource<T>) -> u32 {
            resource.handle.swap(u32::MAX, Relaxed)
        }

        #[doc(hidden)]
        pub fn handle(resource: &Resource<T>) -> u32 {
            resource.handle.load(Relaxed)
        }
    }

    impl<T: WasmResource> fmt::Debug for Resource<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Resource")
                .field("handle", &self.handle)
                .finish()
        }
    }

    impl<T: WasmResource> Drop for Resource<T> {
        fn drop(&mut self) {
            unsafe {
                match self.handle.load(Relaxed) {
                    // If this handle was "taken" then don't do anything in the
                    // destructor.
                    u32::MAX => {}

                    // ... but otherwise do actually destroy it with the imported
                    // component model intrinsic as defined through `T`.
                    other => T::drop(other),
                }
            }
        }
    }
    pub use alloc_crate::alloc;
    pub use alloc_crate::vec::Vec;
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr as *mut u8, layout);
    }
    pub unsafe fn invalid_enum_discriminant<T>() -> T {
        if cfg!(debug_assertions) {
            panic!("invalid enum discriminant")
        } else {
            core::hint::unreachable_unchecked()
        }
    }
    extern crate alloc as alloc_crate;
}

/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]

macro_rules! __export_middleware_impl {
  ($ty:ident) => (self::export!($ty with_types_in self););
  ($ty:ident with_types_in $($path_to_types_root:tt)*) => (
  $($path_to_types_root)*::exports::example::service::handler::__export_example_service_handler_0_1_0_cabi!($ty with_types_in $($path_to_types_root)*::exports::example::service::handler);
  )
}
#[doc(inline)]
pub(crate) use __export_middleware_impl as export;

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.21.0:middleware:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 924] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\x9b\x06\x01A\x02\x01\
A\x0b\x01B\x07\x04\0\x06logger\x03\x01\x01h\0\x01@\x02\x04self\x01\x07messages\x01\
\0\x04\0\x12[method]logger.log\x01\x02\x01i\0\x01@\0\0\x03\x04\0\x0aget-logger\x01\
\x04\x03\x01\x1dexample:service/logging@0.1.0\x05\0\x01B\x17\x04\0\x07request\x03\
\x01\x04\0\x08response\x03\x01\x01m\x01\x0bbad-request\x04\0\x05error\x03\0\x02\x01\
p}\x01o\x02\x04\x04\x01p\x05\x01i\0\x01@\x02\x07headers\x06\x04body\x04\0\x07\x04\
\0\x14[constructor]request\x01\x08\x01h\0\x01@\x01\x04self\x09\0\x06\x04\0\x17[m\
ethod]request.headers\x01\x0a\x01@\x01\x04self\x09\0\x04\x04\0\x14[method]reques\
t.body\x01\x0b\x01i\x01\x01@\x02\x07headers\x06\x04body\x04\0\x0c\x04\0\x15[cons\
tructor]response\x01\x0d\x01h\x01\x01@\x01\x04self\x0e\0\x06\x04\0\x18[method]re\
sponse.headers\x01\x0f\x01@\x01\x04self\x0e\0\x04\x04\0\x15[method]response.body\
\x01\x10\x03\x01\x1bexample:service/types@0.1.0\x05\x01\x02\x03\0\x01\x07request\
\x02\x03\0\x01\x08response\x02\x03\0\x01\x05error\x01B\x0b\x02\x03\x02\x01\x02\x04\
\0\x07request\x03\0\0\x02\x03\x02\x01\x03\x04\0\x08response\x03\0\x02\x02\x03\x02\
\x01\x04\x04\0\x05error\x03\0\x04\x01i\x01\x01i\x03\x01j\x01\x07\x01\x05\x01@\x01\
\x03req\x06\0\x08\x04\0\x07execute\x01\x09\x03\x01\x1dexample:service/handler@0.\
1.0\x05\x05\x01B\x0b\x02\x03\x02\x01\x02\x04\0\x07request\x03\0\0\x02\x03\x02\x01\
\x03\x04\0\x08response\x03\0\x02\x02\x03\x02\x01\x04\x04\0\x05error\x03\0\x04\x01\
i\x01\x01i\x03\x01j\x01\x07\x01\x05\x01@\x01\x03req\x06\0\x08\x04\0\x07execute\x01\
\x09\x04\x01\x1dexample:service/handler@0.1.0\x05\x06\x04\x01\x20example:service\
/middleware@0.1.0\x04\0\x0b\x10\x01\0\x0amiddleware\x03\0\0\0G\x09producers\x01\x0c\
processed-by\x02\x0dwit-component\x070.201.0\x10wit-bindgen-rust\x060.21.0";

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
