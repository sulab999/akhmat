
#[allow(unused_imports)]
#[macro_use]
extern crate log;
extern crate android_logger;

// #[cfg(feature = "android")]
// #[no_mangle]
#[cfg(target_os="android")]
#[allow(non_snake_case,unused_imports,unused_variables,unused_assignments)]
pub mod android{
    extern crate jni;

    use std::{fs, io, thread};
    use std::fs::OpenOptions;
    use std::io::ErrorKind;
    use std::os::fd::{AsRawFd, IntoRawFd};
    use super::*;
    use jni::JNIEnv;
    use jni::objects::{JByteArray, JClass, JObject, JObjectArray, JString, JValue};
    use jni::sys::{jint, jstring};
    use anyhow::{anyhow, Result};
    use std::path::Path;
    use std::process::{Command, Stdio};
    use std::time::Duration;
    use android_logger::{Config};
    use log::{info, LevelFilter};
    use android_properties::{AndroidProperty, getprop, prop_values};
    use pnet::datalink::{self, NetworkInterface};
    use pnet::datalink::Channel::Ethernet;
    use pnet::ipnetwork::IpNetwork;
    use std::net::IpAddr;

    // use btleplug::api::{BDAddr, Central, Manager as _, Peripheral as _};
    // use btleplug::platform::{Manager, Peripheral};

    // use tokio::stream::Stre
    // amExt;
    // use md5::{Digest, Md5};


    #[no_mangle]
    pub extern "system" fn Java_com_android_androidsdk_Nativegolib_hello(
        env: JNIEnv,
        _class: JClass,
    ) -> jstring {
        // 将 Rust 字符串转换为 JNI 字符串
        let result = env.new_string("Hello from Rust!").expect("Couldn't create Java string!");
        // 返回结果
        result.into_raw()
    }
    //检测root路径
    #[no_mangle]
    pub extern "system" fn Java_com_android_androidsdk_Nativegolib_hello1(
        env: JNIEnv,
        _class: JClass,
    ) -> jstring {
        android_logger::init_once(
            Config::default()
                .with_max_level(LevelFilter::Debug)
                .with_tag("akhmatLOG"),
        );
        let paths = [
            "/data/local/",
            "/data/local/bin/",
            "/data/local/xbin/",
            "/sbin/",
            "/su/bin/",
            "/system/bin/",
            "/system/bin/.ext/",
            "/system/bin/failsafe/",
            "/system/sd/xbin/",
            "/system/usr/we-need-root/",
            "/system/xbin/",
            "/cache/",
            "/data/data/",
            "/dev/",
        ];
        let mut result = Default::default();
        // let result = env.new_string("Hello from Rust!").expect("Couldn't create Java string!");
        for path in paths.iter()  {
            if Path::new(path).exists() {
                result = env.new_string("true").expect("path check fail");
                info!("path '{}' exists",path);
                break
            } else {
                result = env.new_string("false").unwrap()
            }
        }
        // info!("Java_com_android_study_useRust_getAppSignature");
        result.into_raw()
    }
    pub fn get_system_property(env: &mut JNIEnv) -> Result<String>{
        let props = prop_values();
        let mut property_value: String = "".to_string();
        for mut prop in props {
            let key = prop.name();
            let value = prop.value();

            // info!("{:?}: {:?}", key, value.unwrap().to_string());
        }
        // info!("{:?}" ,getprop("ro.product.vendor.model").to_string());
        // info!("{:?}" ,getprop("ro.board.platform").to_string());
        // info!("{:?}" ,getprop("ro.build.host").to_string());
        // info!("{:?}" ,getprop("ro.build.flavor").to_string());
        // info!("{:?}" ,getprop("ro.boot.selinux").to_string());
        // info!("{}" ,getprop("ro.debuggable"));
        let selinux = getprop("ro.boot.selinux").value().unwrap().to_string();
        let secure = getprop("ro.secure").value().unwrap().to_string();
        let debuggable = getprop("ro.debuggable").value().unwrap().to_string();
        if secure == String::from("1") && debuggable == String::from("1") {
            property_value = String::from("true");
            // print!("{}", property_value);
        }
        if selinux == String::from("permissive"){
            property_value = String::from("true");
        }

        Ok(property_value)
    }
    //检测系统属性
    #[no_mangle]
    pub extern "system" fn Java_com_android_androidsdk_Nativegolib_hello2(
        mut env: JNIEnv,
        _class: JClass,
    ) -> jstring {

        let syspro = get_system_property(&mut env).expect("property fail");
        // info!("{}" ,syspro);
        let result = env.new_string(syspro).expect("package check fail");
        result.into_raw()
    }

    pub fn get_pkg_name(env: &mut JNIEnv) -> Result<String>{
        let activity_thread_clz = env.find_class("android/app/ActivityThread")?;
        let application_value = env.call_static_method(activity_thread_clz, "currentApplication", "()Landroid/app/Application;", &[])?;
        let application = JObject::try_from(application_value)?;

        //packageName
        let package_name_value = env.call_method(&application, "getPackageName", "()Ljava/lang/String;", &[])?;
        //JValue to JString
        let pkg_name = JString::from(package_name_value.l()?);
        //JString to rust String
        let pkg_name: String = env.get_string(&pkg_name)?.into();
        // info!("{}" ,pkg_name);
        // let mut result = Default::default();
        // result = env.new_string(pkg_name).expect("pkg fail");
        // result.into_raw()
        Ok(pkg_name)
    }
    //检测su文件
    #[no_mangle]
    pub extern "system" fn Java_com_android_androidsdk_Nativegolib_hello3(
        env: JNIEnv,
        _class: JClass,
    ) -> jstring {

        let paths = [
            "/data/local/",
            "/data/local/bin/",
            "/data/local/xbin/",
            "/sbin/",
            "/su/bin/",
            "/system/bin/",
            "/system/bin/.ext/",
            "/system/bin/failsafe/",
            "/system/sd/xbin/",
            "/system/usr/we-need-root/",
            "/system/xbin/",
            "/cache/",
            "/data/",
            "/dev/",
        ];
        let suname = [
            "su",
            "magisk",
            "busybox"
        ];
        let mut result = env.new_string("fail").expect("check fail");

        for path in paths.iter()  {
            for su in suname.iter(){
                let supath = format!("{}{}", path,su);
                if fs::metadata(Path::new(&supath)).is_ok() {
                    info!("{}二进制文件存在。",supath);
                    result = env.new_string("true").expect("su file check fail");
                    break
                }
            }

        }

        result.into_raw()
    }

//执行su
    #[no_mangle]
    pub extern "system" fn Java_com_android_androidsdk_Nativegolib_hello4(
        env: JNIEnv,
        _class: JClass,
    ) -> jstring {

        let mut result = Default::default();

        // let output = Command::new("su")
        //     .arg("-c")
        //     .arg("echo Hello, Android!")
        //     .stdout(Stdio::piped())
        //     .stderr(Stdio::piped())
        //     .output()
        //     .expect("Failed to execute su command");
        let child = Command::new("ls")
            .arg("/data/data")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to spawn su command");
    let child2 = Command::new("mount")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn su command");

        // 等待子进程结束
        let output = child.wait_with_output().expect("Failed  to wait on child");
        let output2 = child2.wait_with_output().expect("Failed  to wait on child2");
        // 检查命令执行的结果

        if output.status.success()  {
            info!("Success: {}", String::from_utf8_lossy(&output.stdout));
            result = env.new_string("true").expect("su check fail");
        } else {
            info!("Error: {}", String::from_utf8_lossy(&output.stderr));
            result = env.new_string("fail").expect("su check fail");
        }
    // if output2.status.success()  {
    //     info!("Success: {}", String::from_utf8_lossy(&output2.stdout));
    //     result = env.new_string("true").expect("su check fail");
    // } else {
    //     info!("Error: {}", String::from_utf8_lossy(&output2.stderr));
    //     result = env.new_string("fail").expect("su check fail");
    // }


    result.into_raw()
    }

    #[no_mangle]
    pub extern "system" fn Java_com_android_androidsdk_Nativegolib_hello5(
        env: JNIEnv,
        _class: JClass,
    ) -> jstring {

        let mut result = env.new_string("fail").expect("check fail");
        // let mac_result = mac_address::;
        // if let Ok(Some(mac)) = mac_result{
        //     info!("MAC address: {:?}", mac.to_string());
        //     result = env.new_string("true").expect("su check fail");
        // }
        let interfaces = datalink::interfaces();

        for interface in interfaces {
            // 获取接口的 MAC 地址
            if let Some(mac) = interface.mac  {
                info!("Interface: {} - MAC Address: {:?}", interface.name,  mac);
                result = env.new_string("true").expect("su check fail");
            }

            // 获取接口的 IP 地址
            for ip_network in interface.ips  {
                match ip_network {
                    IpNetwork::V4(ipv4_network) => {
                        info!("Interface: {} - IPv4 Address: {}", interface.name,  ipv4_network.ip());
                    },
                    IpNetwork::V6(ipv6_network) => {
                        info!("Interface: {} - IPv6 Address: {}", interface.name,  ipv6_network.ip());
                    },
                }
            }
        }


        result.into_raw()
    }

    // use btleplug::api::{Central, CentralEvent, Peripheral};
    // use btleplug::platform::{Adapter, Manager};
    // use std::collections::HashMap;
    // #[tokio::getblue]
    // pub fn getblue()-> Result<String, Box<dyn std::error::Error>> {
        // let manager = Manager::new();
        // let adapters = manager.adapters();
        // let central = adapters.into_iter().nth(0).expect("No  Bluetooth adapters found");
        // let mut blue:String = "".to_string();
        // let properties = central.properties();
        // if let Some(address) = properties.address  {
        //     info!("本机蓝牙 MAC: {:?}", address);
        //     blue = address;
        // }
        //
        // Ok(blue)


    // }
    // fn file_exists(path: &str) -> bool {
    //     match OpenOptions::new().read(true).open(path) {
    //         Ok(_) => true,
    //         Err(e) => {
    //             if let ErrorKind::NotFound = e.kind() {
    //                 false
    //             } else {
    //                 // 其他错误处理
    //                 panic!("发生未预期的错误：{}", e);
    //             }
    //         }
    //     }
    // }

    //模拟器检测
    #[no_mangle]
    pub extern "system" fn Java_com_android_androidsdk_Nativegolib_hello6(
        env: JNIEnv,
        _class: JClass,
    ) -> jstring {

        let paths = [
            "/system/bin/androVM-prop",
            "/system/bin/microvirt-prop",
            "/system/lib/libdroid4x.so",
            "/system/bin/windroyed",
            "/system/bin/nox-prop",
            "/system/lib/libnoxspeedup.so",
            "/system/bin/ttVM-prop",
            "/data/.bluestacks.prop",
            "/system/bin/duosconfig",
            "/system/etc/xxzs_prop.sh",
            "/system/etc/mumu-configs/device-prop-configs/mumu.config",
            "/system/priv-app/ldAppStore",
            "/system/bin/ldinit",
            "/system/bin/ldmountsf",
            "/system/app/AntStore",
            "/system/app/AntLauncher",
            "/vmos.prop",
            "/fstab.titan",
            "/init.titan.rc",
            "/x8.prop",
            "/system/lib/libc_malloc_debug_qemu.so",
            "/system/addon.d",
        ];
        //
        let mut result = env.new_string("fail").expect("moniqi file check fail");

        for path in paths.iter()  {
            // info!("{}",path);
            if fs::metadata(path).is_ok(){
                info!("{}存在模拟器环境。",path);
                result = env.new_string("true").expect("moniqi file check fail");
                break
            }

        }

        result.into_raw()
    }
    //检测frida
    #[no_mangle]
    pub extern "system" fn Java_com_android_androidsdk_Nativegolib_hello7(
        env: JNIEnv,
        _class: JClass,
    ) -> jstring {

        let mut result = env.new_string("fail").expect("check fail");


        // info!("package name = {}", pkg_name);
        let child = Command::new("service")
            .arg("list")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed command");


        // 等待子进程结束
        let output = child.wait_with_output().expect("Failed  to wait on child");

        // 检查命令执行的结果

        if output.status.success()  {
            let output_str = String::from_utf8_lossy(&output.stdout);
            if output_str.contains("frida")  {
                info!("Output contains 'frida'");
                result = env.new_string("true").expect("frida check fail");
            }

        } else {
            info!("Error: {}", String::from_utf8_lossy(&output.stderr));
            result = env.new_string("fail").expect("frida check fail");
        }
        result.into_raw()
    }
}
