#[cfg(any(feature = "v0_4_2"))]
use crate::plugin::LookupDirs;
#[cfg(feature = "lua")]
use crate::{lua::ToLuaTable, plugin::ComponentLoader};
use {
	crate::{prelude::*, Core, InitFlags, Properties},
	glib::{MainContext, MainLoop},
	pipewire_sys::{pw_context, pw_core},
};

impl Core {
	#[doc(alias = "wp_init")]
	pub fn init_with_flags(flags: InitFlags) {
		unsafe { ffi::wp_init(flags.into_glib()) }
	}

	#[doc(alias = "wp_init")]
	pub fn init() {
		Self::init_with_flags(InitFlags::ALL)
	}

	#[cfg(feature = "v0_4_12")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v0_4_12")))]
	#[doc(alias = "wp_get_library_version")]
	pub fn library_version() -> String {
		unsafe { from_glib_full(ffi::wp_get_library_version()) }
	}

	#[cfg(feature = "v0_4_12")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v0_4_12")))]
	#[doc(alias = "wp_get_library_api_version")]
	pub fn library_api_version() -> String {
		unsafe { from_glib_full(ffi::wp_get_library_api_version()) }
	}

	#[doc(alias = "wp_get_module_dir")]
	pub fn module_dir() -> String {
		unsafe { from_glib_full(ffi::wp_get_module_dir()) }
	}

	#[cfg_attr(feature = "v0_4_2", deprecated = "use `find_file` instead")]
	#[doc(alias = "wp_get_config_dir")]
	pub fn config_dir() -> String {
		unsafe { from_glib_full(ffi::wp_get_config_dir()) }
	}

	#[cfg_attr(feature = "v0_4_2", deprecated = "use `find_file` instead")]
	#[doc(alias = "wp_get_data_dir")]
	pub fn data_dir() -> String {
		unsafe { from_glib_full(ffi::wp_get_data_dir()) }
	}

	#[cfg(feature = "v0_4_2")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v0_4_2")))]
	#[doc(alias = "wp_find_file")]
	pub fn find_file(dirs: LookupDirs, filename: &str, subdir: Option<&str>) -> Option<String> {
		unsafe {
			from_glib_full(ffi::wp_find_file(
				dirs.into_glib(),
				filename.to_glib_none().0,
				subdir.to_glib_none().0,
			))
		}
	}

	#[cfg(feature = "v0_4_2")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v0_4_2")))]
	#[doc(alias = "wp_new_files_iterator")]
	pub fn find_files(dirs: LookupDirs, subdir: Option<&str>, suffix: Option<&str>) -> IntoValueIterator<String> {
		unsafe {
			IntoValueIterator::with_inner(from_glib_full(ffi::wp_new_files_iterator(
				dirs.into_glib(),
				subdir.to_glib_none().0,
				suffix.to_glib_none().0,
			)))
		}
	}

	#[doc(alias = "wp_core_clone")]
	pub fn clone_context(&self) -> Option<Self> {
		unsafe { from_glib_full(ffi::wp_core_clone(self.to_glib_none().0)) }
	}

	#[doc(alias = "wp_core_get_g_main_context")]
	#[doc(alias = "get_g_main_context")]
	pub fn default_context(&self) -> MainContext {
		self
			.g_main_context()
			.unwrap_or_else(|| MainContext::ref_thread_default())
	}

	#[doc(alias = "wp_core_get_pw_core")]
	#[doc(alias = "get_pw_core")]
	pub fn pw_core_raw(&self) -> *mut pw_core {
		unsafe { ffi::wp_core_get_pw_core(self.to_glib_none().0) }
	}

	#[doc(alias = "wp_core_get_pw_context")]
	#[doc(alias = "get_pw_context")]
	pub fn pw_context_raw(&self) -> NonNull<pw_context> {
		unsafe { NonNull::new(ffi::wp_core_get_pw_context(self.to_glib_none().0)).expect("pw_context for WpCore") }
	}

	#[cfg(feature = "lua")]
	#[cfg_attr(docsrs, doc(cfg(feature = "lua")))]
	#[doc(alias = "wp_core_load_component")]
	pub fn load_lua_script<A: ToLuaTable>(&self, script_path: &str, args: A) -> Result<(), Error> {
		let args = args
			.to_lua_variant()
			.and_then(|v| v.map(|v| v.into_vardict()).transpose())?;
		self.load_component(
			script_path,
			ComponentLoader::TYPE_LUA_SCRIPT,
			args.as_ref().map(|v| v.as_variant()),
		)
	}

	#[cfg(feature = "futures")]
	#[cfg_attr(docsrs, doc(cfg(feature = "futures")))]
	#[doc(alias = "wp_core_connect")]
	#[doc(alias = "connect")]
	pub fn connect_future(&self) -> impl Future<Output = Result<(), Error>> {
		use crate::util::futures::signal_once;

		let connect = signal_once(match () {
			#[cfg(feature = "glib-signal")]
			() => self.signal_stream(Self::SIGNAL_CONNECTED),
			#[cfg(not(feature = "glib-signal"))]
			() => |handler| self.connect_connected(handler),
		});

		let res = match self.connect() {
			true => Ok(connect),
			false => Err(Error::new(
				LibraryErrorEnum::OperationFailed,
				"failed to connect to pipewire",
			)),
		};

		async move {
			match res {
				Ok(connect) => connect.await,
				Err(e) => Err(e),
			}
		}
	}

	pub fn run<F: FnOnce(&MainContext, MainLoop, Core)>(props: Option<Properties>, setup: F) {
		let mainloop = MainLoop::new(None, false);
		let context = mainloop.context();
		let core = context
			.with_thread_default(|| {
				let core = Core::new(Some(&context), props);
				let _disconnect_handler = core.connect_disconnected({
					let mainloop = mainloop.clone();
					move |_core| mainloop.quit()
				});

				setup(&context, mainloop.clone(), core.clone());

				mainloop.run();

				core
			})
			.unwrap();

		core.disconnect();
	}
}

#[test]
#[cfg(any(feature = "v0_4_2"))]
fn wp_new_files_iterator() {
	let file = Core::find_file(LookupDirs::PREFIX_SHARE, "create-item.lua", Some("scripts"));
	assert!(file.is_some());

	let files = Core::find_files(LookupDirs::PREFIX_SHARE, None, Some(".conf")).into_iter();
	assert_ne!(0, files.count());
}
