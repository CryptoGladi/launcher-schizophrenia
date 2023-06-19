use crate::game::SPathBuf;
use super::GameManager;

fn get_const_vec_flags(flags: &mut Vec<String>) {
    //flags.push(
    //    "/home/gladi/.tlauncher/mojang_jre/java-runtime-beta/linux/java-runtime-beta/bin/java".to_string(),
    //);
    flags.push("-XX:+UnlockExperimentalVMOptions".to_string());
    flags.push("-XX:+UseG1GC".to_string());
    flags.push("-XX:G1NewSizePercent=20".to_string());
    flags.push("-XX:G1ReservePercent=20".to_string());
    flags.push("-XX:MaxGCPauseMillis=50".to_string());
    flags.push("-XX:G1HeapRegionSize=32M".to_string());
    flags.push("-XX:+DisableExplicitGC".to_string());
    flags.push("-XX:+AlwaysPreTouch".to_string());
    flags.push("-XX:+ParallelRefProcEnabled".to_string());
    // flags.push("-Xms2048M".to_string());
    // flags.push("-Xmx2048M".to_string());
    flags.push("-Dfile.encoding=UTF-8".to_string());
    // flags.push("-Djava.library.path=/home/gladi/Minecraft/versions/ForgeOptiFine 1.18.2/natives".to_string());
    flags.push("-Dminecraft.launcher.brand=java-minecraft-launcher".to_string());
    flags.push("-Dminecraft.launcher.version=1.6.84-j".to_string());
    // flags.push("-cp".to_string());
    // flags.push("/home/gladi/Minecraft/libraries/pw/modder/transformerDiscoverer/2.2/transformerDiscoverer-2.2.jar:/home/gladi/Minecraft/libraries/cpw/mods/securejarhandler/1.0.8/securejarhandler-1.0.8.jar:/home/gladi/Minecraft/libraries/org/ow2/asm/asm/9.5/asm-9.5.jar:/home/gladi/Minecraft/libraries/org/ow2/asm/asm-commons/9.5/asm-commons-9.5.jar:/home/gladi/Minecraft/libraries/org/ow2/asm/asm-tree/9.5/asm-tree-9.5.jar:/home/gladi/Minecraft/libraries/org/ow2/asm/asm-util/9.5/asm-util-9.5.jar:/home/gladi/Minecraft/libraries/org/ow2/asm/asm-analysis/9.5/asm-analysis-9.5.jar:/home/gladi/Minecraft/libraries/net/minecraftforge/accesstransformers/8.0.4/accesstransformers-8.0.4.jar:/home/gladi/Minecraft/libraries/org/antlr/antlr4-runtime/4.9.1/antlr4-runtime-4.9.1.jar:/home/gladi/Minecraft/libraries/net/minecraftforge/eventbus/5.0.3/eventbus-5.0.3.jar:/home/gladi/Minecraft/libraries/net/minecraftforge/forgespi/4.0.15-4.x/forgespi-4.0.15-4.x.jar:/home/gladi/Minecraft/libraries/net/minecraftforge/coremods/5.0.1/coremods-5.0.1.jar:/home/gladi/Minecraft/libraries/cpw/mods/modlauncher/9.1.3/modlauncher-9.1.3.jar:/home/gladi/Minecraft/libraries/net/minecraftforge/unsafe/0.2.0/unsafe-0.2.0.jar:/home/gladi/Minecraft/libraries/com/electronwill/night-config/core/3.6.4/core-3.6.4.jar:/home/gladi/Minecraft/libraries/com/electronwill/night-config/toml/3.6.4/toml-3.6.4.jar:/home/gladi/Minecraft/libraries/org/apache/maven/maven-artifact/3.6.3/maven-artifact-3.6.3.jar:/home/gladi/Minecraft/libraries/net/jodah/typetools/0.8.3/typetools-0.8.3.jar:/home/gladi/Minecraft/libraries/net/minecrell/terminalconsoleappender/1.2.0/terminalconsoleappender-1.2.0.jar:/home/gladi/Minecraft/libraries/org/jline/jline-reader/3.12.1/jline-reader-3.12.1.jar:/home/gladi/Minecraft/libraries/org/jline/jline-terminal/3.12.1/jline-terminal-3.12.1.jar:/home/gladi/Minecraft/libraries/org/spongepowered/mixin/0.8.5/mixin-0.8.5.jar:/home/gladi/Minecraft/libraries/org/openjdk/nashorn/nashorn-core/15.3/nashorn-core-15.3.jar:/home/gladi/Minecraft/libraries/net/minecraftforge/JarJarSelector/0.3.19/JarJarSelector-0.3.19.jar:/home/gladi/Minecraft/libraries/net/minecraftforge/JarJarMetadata/0.3.19/JarJarMetadata-0.3.19.jar:/home/gladi/Minecraft/libraries/cpw/mods/bootstraplauncher/1.0.0/bootstraplauncher-1.0.0.jar:/home/gladi/Minecraft/libraries/net/minecraftforge/JarJarFileSystems/0.3.19/JarJarFileSystems-0.3.19.jar:/home/gladi/Minecraft/libraries/net/minecraftforge/fmlloader/1.18.2-40.2.9/fmlloader-1.18.2-40.2.9.jar:/home/gladi/Minecraft/libraries/com/mojang/logging/1.0.0/logging-1.0.0.jar:/home/gladi/Minecraft/libraries/com/mojang/blocklist/1.0.10/blocklist-1.0.10.jar:/home/gladi/Minecraft/libraries/ru/tln4/empty/0.1/empty-0.1.jar:/home/gladi/Minecraft/libraries/com/github/oshi/oshi-core/5.8.5/oshi-core-5.8.5.jar:/home/gladi/Minecraft/libraries/net/java/dev/jna/jna/5.10.0/jna-5.10.0.jar:/home/gladi/Minecraft/libraries/net/java/dev/jna/jna-platform/5.10.0/jna-platform-5.10.0.jar:/home/gladi/Minecraft/libraries/org/slf4j/slf4j-api/1.8.0-beta4/slf4j-api-1.8.0-beta4.jar:/home/gladi/Minecraft/libraries/org/apache/logging/log4j/log4j-slf4j18-impl/2.17.0/log4j-slf4j18-impl-2.17.0.jar:/home/gladi/Minecraft/libraries/com/ibm/icu/icu4j/70.1/icu4j-70.1.jar:/home/gladi/Minecraft/libraries/com/mojang/javabridge/1.2.24/javabridge-1.2.24.jar:/home/gladi/Minecraft/libraries/net/sf/jopt-simple/jopt-simple/5.0.4/jopt-simple-5.0.4.jar:/home/gladi/Minecraft/libraries/io/netty/netty-all/4.1.68.Final/netty-all-4.1.68.Final.jar:/home/gladi/Minecraft/libraries/com/google/guava/failureaccess/1.0.1/failureaccess-1.0.1.jar:/home/gladi/Minecraft/libraries/com/google/guava/guava/31.0.1-jre/guava-31.0.1-jre.jar:/home/gladi/Minecraft/libraries/org/apache/commons/commons-lang3/3.12.0/commons-lang3-3.12.0.jar:/home/gladi/Minecraft/libraries/commons-io/commons-io/2.11.0/commons-io-2.11.0.jar:/home/gladi/Minecraft/libraries/commons-codec/commons-codec/1.15/commons-codec-1.15.jar:/home/gladi/Minecraft/libraries/com/mojang/brigadier/1.0.18/brigadier-1.0.18.jar:/home/gladi/Minecraft/libraries/com/mojang/datafixerupper/4.1.27/datafixerupper-4.1.27.jar:/home/gladi/Minecraft/libraries/com/google/code/gson/gson/2.8.9/gson-2.8.9.jar:/home/gladi/Minecraft/libraries/by/ely/authlib/3.11.49.2/authlib-3.11.49.2.jar:/home/gladi/Minecraft/libraries/org/apache/commons/commons-compress/1.21/commons-compress-1.21.jar:/home/gladi/Minecraft/libraries/org/apache/httpcomponents/httpclient/4.5.13/httpclient-4.5.13.jar:/home/gladi/Minecraft/libraries/commons-logging/commons-logging/1.2/commons-logging-1.2.jar:/home/gladi/Minecraft/libraries/org/apache/httpcomponents/httpcore/4.4.14/httpcore-4.4.14.jar:/home/gladi/Minecraft/libraries/it/unimi/dsi/fastutil/8.5.6/fastutil-8.5.6.jar:/home/gladi/Minecraft/libraries/org/apache/logging/log4j/log4j-api/2.17.0/log4j-api-2.17.0.jar:/home/gladi/Minecraft/libraries/org/apache/logging/log4j/log4j-core/2.17.0/log4j-core-2.17.0.jar:/home/gladi/Minecraft/libraries/org/lwjgl/lwjgl/3.2.2/lwjgl-3.2.2.jar:/home/gladi/Minecraft/libraries/org/lwjgl/lwjgl-jemalloc/3.2.2/lwjgl-jemalloc-3.2.2.jar:/home/gladi/Minecraft/libraries/org/lwjgl/lwjgl-openal/3.2.2/lwjgl-openal-3.2.2.jar:/home/gladi/Minecraft/libraries/org/lwjgl/lwjgl-opengl/3.2.2/lwjgl-opengl-3.2.2.jar:/home/gladi/Minecraft/libraries/org/lwjgl/lwjgl-glfw/3.2.2/lwjgl-glfw-3.2.2.jar:/home/gladi/Minecraft/libraries/org/lwjgl/lwjgl-stb/3.2.2/lwjgl-stb-3.2.2.jar:/home/gladi/Minecraft/libraries/org/lwjgl/lwjgl-tinyfd/3.2.2/lwjgl-tinyfd-3.2.2.jar:/home/gladi/Minecraft/libraries/com/mojang/text2speech/1.12.4/text2speech-1.12.4.jar:/home/gladi/Minecraft/versions/ForgeOptiFine 1.18.2/ForgeOptiFine 1.18.2.jar".to_string());
    flags.push("-Djava.net.preferIPv6Addresses=system".to_string());
    flags.push("-DignoreList=bootstraplauncher,securejarhandler,asm-commons,asm-util,asm-analysis,asm-tree,asm,JarJarFileSystems,client-extra,fmlcore,javafmllanguage,lowcodelanguage,mclanguage,forge-,ForgeOptiFine 1.18.2.jar".to_string());
    flags.push("-DmergeModules=jna-5.10.0.jar,jna-platform-5.10.0.jar,java-objc-bridge-1.0.0.jar".to_string());
    // flags.push("-DlibraryDirectory=/home/gladi/Minecraft/libraries");
    // flags.push("-p".to_string());
    // flags.push("/home/gladi/Minecraft/libraries/cpw/mods/bootstraplauncher/1.0.0/bootstraplauncher-1.0.0.jar:/home/gladi/Minecraft/libraries/cpw/mods/securejarhandler/1.0.8/securejarhandler-1.0.8.jar:/home/gladi/Minecraft/libraries/org/ow2/asm/asm-commons/9.5/asm-commons-9.5.jar:/home/gladi/Minecraft/libraries/org/ow2/asm/asm-util/9.5/asm-util-9.5.jar:/home/gladi/Minecraft/libraries/org/ow2/asm/asm-analysis/9.5/asm-analysis-9.5.jar:/home/gladi/Minecraft/libraries/org/ow2/asm/asm-tree/9.5/asm-tree-9.5.jar:/home/gladi/Minecraft/libraries/org/ow2/asm/asm/9.5/asm-9.5.jar:/home/gladi/Minecraft/libraries/net/minecraftforge/JarJarFileSystems/0.3.19/JarJarFileSystems-0.3.19.jar");
    flags.push("--add-modules".to_string());
    flags.push("ALL-MODULE-PATH".to_string());
    flags.push("--add-opens".to_string());
    flags.push("java.base/java.util.jar=cpw.mods.securejarhandler".to_string());
    flags.push("--add-opens".to_string());
    flags.push("java.base/java.lang.invoke=cpw.mods.securejarhandler".to_string());
    flags.push("--add-exports".to_string());
    flags.push("java.base/sun.security.util=cpw.mods.securejarhandler".to_string());
    flags.push("--add-exports".to_string());
    flags.push("jdk.naming.dns/com.sun.jndi.dns=java.naming".to_string());
    flags.push("-Dtransformers.list=optifine:OptiFine:1.18.2_HD_U_H9_pre2".to_string());
    flags.push("-Xss2M".to_string());
    flags.push("cpw.mods.bootstraplauncher.BootstrapLauncher".to_string());
    // flags.push("--username");
    // flags.push("llll");
    flags.push("--version".to_string());
    flags.push("ForgeOptiFine 1.18.2".to_string());
    // flags.push("--gameDir");
    // flags.push("/home/gladi/Minecraft");
    // flags.push("--assetsDir");
    // flags.push("/home/gladi/Minecraft/assets");
    flags.push("--assetIndex".to_string());
    flags.push("1.18".to_string());
    flags.push("--uuid".to_string());
    flags.push("4b532043ac8b31818790e4f95d288ce5".to_string());
    flags.push("--accessToken".to_string());
    flags.push("4b532043ac8b31818790e4f95d288ce5".to_string());
    flags.push("--clientId".to_string());
    flags.push("".to_string());
    flags.push("--xuid".to_string());
    flags.push("".to_string());
    flags.push("--userType".to_string());
    flags.push("legacy".to_string());
    flags.push("--versionType".to_string());
    flags.push("modified".to_string());
    flags.push("--width".to_string());
    flags.push("925".to_string());
    flags.push("--height".to_string());
    flags.push("530".to_string());
    flags.push("--launchTarget".to_string());
    flags.push("forgeclient".to_string());
    flags.push("--fml.forgeVersion".to_string());
    flags.push("40.2.9".to_string());
    flags.push("--fml.mcVersion".to_string());
    flags.push("1.18.2".to_string());
    flags.push("--fml.forgeGroup".to_string());
    flags.push("net.minecraftforge".to_string());
    flags.push("--fml.mcpVersion".to_string());
    flags.push("20220404.173914".to_string());
}

fn get_vec_library(game: &GameManager, flags: &mut Vec<String>) {
    let p = SPathBuf(game.path_to_minecraft.0.join("libraries")).to_string();

   // "/home/gladi/Minecraft/libraries/cpw/mods/bootstraplauncher/1.0.0/bootstraplauncher-1.0.0.jar:/home/gladi/Minecraft/libraries/cpw/mods/securejarhandler/1.0.8/securejarhandler-1.0.8.jar:/home/gladi/Minecraft/libraries/org/ow2/asm/asm-commons/9.5/asm-commons-9.5.jar:/home/gladi/Minecraft/libraries/org/ow2/asm/asm-util/9.5/asm-util-9.5.jar:/home/gladi/Minecraft/libraries/org/ow2/asm/asm-analysis/9.5/asm-analysis-9.5.jar:/home/gladi/Minecraft/libraries/org/ow2/asm/asm-tree/9.5/asm-tree-9.5.jar:/home/gladi/Minecraft/libraries/org/ow2/asm/asm/9.5/asm-9.5.jar:/home/gladi/Minecraft/libraries/net/minecraftforge/JarJarFileSystems/0.3.19/JarJarFileSystems-0.3.19.jar"
   flags.push("-p".to_string());
   flags.push(format!("{}/cpw/mods/bootstraplauncher/1.0.0/bootstraplauncher-1.0.0.jar:{}/cpw/mods/securejarhandler/1.0.8/securejarhandler-1.0.8.jar:{}/org/ow2/asm/asm-commons/9.5/asm-commons-9.5.jar:{}/org/ow2/asm/asm-util/9.5/asm-util-9.5.jar:{}/org/ow2/asm/asm-analysis/9.5/asm-analysis-9.5.jar:{}/org/ow2/asm/asm-tree/9.5/asm-tree-9.5.jar:{}/org/ow2/asm/asm/9.5/asm-9.5.jar:{}/net/minecraftforge/JarJarFileSystems/0.3.19/JarJarFileSystems-0.3.19.jar", p, p, p, p, p, p, p, p));

   // "-Djava.library.path=/home/gladi/Minecraft/versions/ForgeOptiFine 1.18.2/natives"
   flags.push(format!("-Djava.library.path={:?}/versions/ForgeOptiFine 1.18.2/natives", game.path_to_java));

   flags.push(format!("-DlibraryDirectory={:?}", p));

    // /home/gladi/Minecraft/libraries/pw/modder/transformerDiscoverer/2.2/transformerDiscoverer-2.2.jar:/home/gladi/Minecraft/libraries/cpw/mods/securejarhandler/1.0.8/securejarhandler-1.0.8.jar:/home/gladi/Minecraft/libraries/org/ow2/asm/asm/9.5/asm-9.5.jar:/home/gladi/Minecraft/libraries/org/ow2/asm/asm-commons/9.5/asm-commons-9.5.jar:/home/gladi/Minecraft/libraries/org/ow2/asm/asm-tree/9.5/asm-tree-9.5.jar:/home/gladi/Minecraft/libraries/org/ow2/asm/asm-util/9.5/asm-util-9.5.jar:/home/gladi/Minecraft/libraries/org/ow2/asm/asm-analysis/9.5/asm-analysis-9.5.jar:/home/gladi/Minecraft/libraries/net/minecraftforge/accesstransformers/8.0.4/accesstransformers-8.0.4.jar:/home/gladi/Minecraft/libraries/org/antlr/antlr4-runtime/4.9.1/antlr4-runtime-4.9.1.jar:/home/gladi/Minecraft/libraries/net/minecraftforge/eventbus/5.0.3/eventbus-5.0.3.jar:/home/gladi/Minecraft/libraries/net/minecraftforge/forgespi/4.0.15-4.x/forgespi-4.0.15-4.x.jar:/home/gladi/Minecraft/libraries/net/minecraftforge/coremods/5.0.1/coremods-5.0.1.jar:/home/gladi/Minecraft/libraries/cpw/mods/modlauncher/9.1.3/modlauncher-9.1.3.jar:/home/gladi/Minecraft/libraries/net/minecraftforge/unsafe/0.2.0/unsafe-0.2.0.jar:/home/gladi/Minecraft/libraries/com/electronwill/night-config/core/3.6.4/core-3.6.4.jar:/home/gladi/Minecraft/libraries/com/electronwill/night-config/toml/3.6.4/toml-3.6.4.jar:/home/gladi/Minecraft/libraries/org/apache/maven/maven-artifact/3.6.3/maven-artifact-3.6.3.jar:/home/gladi/Minecraft/libraries/net/jodah/typetools/0.8.3/typetools-0.8.3.jar:/home/gladi/Minecraft/libraries/net/minecrell/terminalconsoleappender/1.2.0/terminalconsoleappender-1.2.0.jar:/home/gladi/Minecraft/libraries/org/jline/jline-reader/3.12.1/jline-reader-3.12.1.jar:/home/gladi/Minecraft/libraries/org/jline/jline-terminal/3.12.1/jline-terminal-3.12.1.jar:/home/gladi/Minecraft/libraries/org/spongepowered/mixin/0.8.5/mixin-0.8.5.jar:/home/gladi/Minecraft/libraries/org/openjdk/nashorn/nashorn-core/15.3/nashorn-core-15.3.jar:/home/gladi/Minecraft/libraries/net/minecraftforge/JarJarSelector/0.3.19/JarJarSelector-0.3.19.jar:/home/gladi/Minecraft/libraries/net/minecraftforge/JarJarMetadata/0.3.19/JarJarMetadata-0.3.19.jar:/home/gladi/Minecraft/libraries/cpw/mods/bootstraplauncher/1.0.0/bootstraplauncher-1.0.0.jar:/home/gladi/Minecraft/libraries/net/minecraftforge/JarJarFileSystems/0.3.19/JarJarFileSystems-0.3.19.jar:/home/gladi/Minecraft/libraries/net/minecraftforge/fmlloader/1.18.2-40.2.9/fmlloader-1.18.2-40.2.9.jar:/home/gladi/Minecraft/libraries/com/mojang/logging/1.0.0/logging-1.0.0.jar:/home/gladi/Minecraft/libraries/com/mojang/blocklist/1.0.10/blocklist-1.0.10.jar:/home/gladi/Minecraft/libraries/ru/tln4/empty/0.1/empty-0.1.jar:/home/gladi/Minecraft/libraries/com/github/oshi/oshi-core/5.8.5/oshi-core-5.8.5.jar:/home/gladi/Minecraft/libraries/net/java/dev/jna/jna/5.10.0/jna-5.10.0.jar:/home/gladi/Minecraft/libraries/net/java/dev/jna/jna-platform/5.10.0/jna-platform-5.10.0.jar:/home/gladi/Minecraft/libraries/org/slf4j/slf4j-api/1.8.0-beta4/slf4j-api-1.8.0-beta4.jar:/home/gladi/Minecraft/libraries/org/apache/logging/log4j/log4j-slf4j18-impl/2.17.0/log4j-slf4j18-impl-2.17.0.jar:/home/gladi/Minecraft/libraries/com/ibm/icu/icu4j/70.1/icu4j-70.1.jar:/home/gladi/Minecraft/libraries/com/mojang/javabridge/1.2.24/javabridge-1.2.24.jar:/home/gladi/Minecraft/libraries/net/sf/jopt-simple/jopt-simple/5.0.4/jopt-simple-5.0.4.jar:/home/gladi/Minecraft/libraries/io/netty/netty-all/4.1.68.Final/netty-all-4.1.68.Final.jar:/home/gladi/Minecraft/libraries/com/google/guava/failureaccess/1.0.1/failureaccess-1.0.1.jar:/home/gladi/Minecraft/libraries/com/google/guava/guava/31.0.1-jre/guava-31.0.1-jre.jar:/home/gladi/Minecraft/libraries/org/apache/commons/commons-lang3/3.12.0/commons-lang3-3.12.0.jar:/home/gladi/Minecraft/libraries/commons-io/commons-io/2.11.0/commons-io-2.11.0.jar:/home/gladi/Minecraft/libraries/commons-codec/commons-codec/1.15/commons-codec-1.15.jar:/home/gladi/Minecraft/libraries/com/mojang/brigadier/1.0.18/brigadier-1.0.18.jar:/home/gladi/Minecraft/libraries/com/mojang/datafixerupper/4.1.27/datafixerupper-4.1.27.jar:/home/gladi/Minecraft/libraries/com/google/code/gson/gson/2.8.9/gson-2.8.9.jar:/home/gladi/Minecraft/libraries/by/ely/authlib/3.11.49.2/authlib-3.11.49.2.jar:/home/gladi/Minecraft/libraries/org/apache/commons/commons-compress/1.21/commons-compress-1.21.jar:/home/gladi/Minecraft/libraries/org/apache/httpcomponents/httpclient/4.5.13/httpclient-4.5.13.jar:/home/gladi/Minecraft/libraries/commons-logging/commons-logging/1.2/commons-logging-1.2.jar:/home/gladi/Minecraft/libraries/org/apache/httpcomponents/httpcore/4.4.14/httpcore-4.4.14.jar:/home/gladi/Minecraft/libraries/it/unimi/dsi/fastutil/8.5.6/fastutil-8.5.6.jar:/home/gladi/Minecraft/libraries/org/apache/logging/log4j/log4j-api/2.17.0/log4j-api-2.17.0.jar:/home/gladi/Minecraft/libraries/org/apache/logging/log4j/log4j-core/2.17.0/log4j-core-2.17.0.jar:/home/gladi/Minecraft/libraries/org/lwjgl/lwjgl/3.2.2/lwjgl-3.2.2.jar:/home/gladi/Minecraft/libraries/org/lwjgl/lwjgl-jemalloc/3.2.2/lwjgl-jemalloc-3.2.2.jar:/home/gladi/Minecraft/libraries/org/lwjgl/lwjgl-openal/3.2.2/lwjgl-openal-3.2.2.jar:/home/gladi/Minecraft/libraries/org/lwjgl/lwjgl-opengl/3.2.2/lwjgl-opengl-3.2.2.jar:/home/gladi/Minecraft/libraries/org/lwjgl/lwjgl-glfw/3.2.2/lwjgl-glfw-3.2.2.jar:/home/gladi/Minecraft/libraries/org/lwjgl/lwjgl-stb/3.2.2/lwjgl-stb-3.2.2.jar:/home/gladi/Minecraft/libraries/org/lwjgl/lwjgl-tinyfd/3.2.2/lwjgl-tinyfd-3.2.2.jar:/home/gladi/Minecraft/libraries/com/mojang/text2speech/1.12.4/text2speech-1.12.4.jar:/home/gladi/Minecraft/versions/ForgeOptiFine 1.18.2/ForgeOptiFine 1.18.2.jar"
   flags.push("-cp".to_string());
   flags.push(format!("{}/pw/modder/transformerDiscoverer/2.2/transformerDiscoverer-2.2.jar:{}/cpw/mods/securejarhandler/1.0.8/securejarhandler-1.0.8.jar:{}/org/ow2/asm/asm/9.5/asm-9.5.jar:{}/org/ow2/asm/asm-commons/9.5/asm-commons-9.5.jar:{}/org/ow2/asm/asm-tree/9.5/asm-tree-9.5.jar:{}/org/ow2/asm/asm-util/9.5/asm-util-9.5.jar:{}/org/ow2/asm/asm-analysis/9.5/asm-analysis-9.5.jar:{}/net/minecraftforge/accesstransformers/8.0.4/accesstransformers-8.0.4.jar:{}/org/antlr/antlr4-runtime/4.9.1/antlr4-runtime-4.9.1.jar:{}/net/minecraftforge/eventbus/5.0.3/eventbus-5.0.3.jar:{}/net/minecraftforge/forgespi/4.0.15-4.x/forgespi-4.0.15-4.x.jar:{}/net/minecraftforge/coremods/5.0.1/coremods-5.0.1.jar:{}/cpw/mods/modlauncher/9.1.3/modlauncher-9.1.3.jar:{}/net/minecraftforge/unsafe/0.2.0/unsafe-0.2.0.jar:{}/com/electronwill/night-config/core/3.6.4/core-3.6.4.jar:{}/com/electronwill/night-config/toml/3.6.4/toml-3.6.4.jar:{}/org/apache/maven/maven-artifact/3.6.3/maven-artifact-3.6.3.jar:{}/net/jodah/typetools/0.8.3/typetools-0.8.3.jar:{}/net/minecrell/terminalconsoleappender/1.2.0/terminalconsoleappender-1.2.0.jar:{}/org/jline/jline-reader/3.12.1/jline-reader-3.12.1.jar:{}/org/jline/jline-terminal/3.12.1/jline-terminal-3.12.1.jar:{}/org/spongepowered/mixin/0.8.5/mixin-0.8.5.jar:{}/org/openjdk/nashorn/nashorn-core/15.3/nashorn-core-15.3.jar:{}/net/minecraftforge/JarJarSelector/0.3.19/JarJarSelector-0.3.19.jar:{}/net/minecraftforge/JarJarMetadata/0.3.19/JarJarMetadata-0.3.19.jar:{}/cpw/mods/bootstraplauncher/1.0.0/bootstraplauncher-1.0.0.jar:{}/net/minecraftforge/JarJarFileSystems/0.3.19/JarJarFileSystems-0.3.19.jar:{}/net/minecraftforge/fmlloader/1.18.2-40.2.9/fmlloader-1.18.2-40.2.9.jar:{}/com/mojang/logging/1.0.0/logging-1.0.0.jar:{}/com/mojang/blocklist/1.0.10/blocklist-1.0.10.jar:{}/ru/tln4/empty/0.1/empty-0.1.jar:{}/com/github/oshi/oshi-core/5.8.5/oshi-core-5.8.5.jar:{}/net/java/dev/jna/jna/5.10.0/jna-5.10.0.jar:{}/net/java/dev/jna/jna-platform/5.10.0/jna-platform-5.10.0.jar:{}/org/slf4j/slf4j-api/1.8.0-beta4/slf4j-api-1.8.0-beta4.jar:{}/org/apache/logging/log4j/log4j-slf4j18-impl/2.17.0/log4j-slf4j18-impl-2.17.0.jar:{}/com/ibm/icu/icu4j/70.1/icu4j-70.1.jar:{}/com/mojang/javabridge/1.2.24/javabridge-1.2.24.jar:{}/net/sf/jopt-simple/jopt-simple/5.0.4/jopt-simple-5.0.4.jar:{}/io/netty/netty-all/4.1.68.Final/netty-all-4.1.68.Final.jar:{}/com/google/guava/failureaccess/1.0.1/failureaccess-1.0.1.jar:{}/com/google/guava/guava/31.0.1-jre/guava-31.0.1-jre.jar:{}/org/apache/commons/commons-lang3/3.12.0/commons-lang3-3.12.0.jar:{}/commons-io/commons-io/2.11.0/commons-io-2.11.0.jar:{}/commons-codec/commons-codec/1.15/commons-codec-1.15.jar:{}/com/mojang/brigadier/1.0.18/brigadier-1.0.18.jar:{}/com/mojang/datafixerupper/4.1.27/datafixerupper-4.1.27.jar:{}/com/google/code/gson/gson/2.8.9/gson-2.8.9.jar:{}/by/ely/authlib/3.11.49.2/authlib-3.11.49.2.jar:{}/org/apache/commons/commons-compress/1.21/commons-compress-1.21.jar:{}/org/apache/httpcomponents/httpclient/4.5.13/httpclient-4.5.13.jar:{}/commons-logging/commons-logging/1.2/commons-logging-1.2.jar:{}/org/apache/httpcomponents/httpcore/4.4.14/httpcore-4.4.14.jar:{}/it/unimi/dsi/fastutil/8.5.6/fastutil-8.5.6.jar:{}/org/apache/logging/log4j/log4j-api/2.17.0/log4j-api-2.17.0.jar:{}/org/apache/logging/log4j/log4j-core/2.17.0/log4j-core-2.17.0.jar:{}/org/lwjgl/lwjgl/3.2.2/lwjgl-3.2.2.jar:{}/org/lwjgl/lwjgl-jemalloc/3.2.2/lwjgl-jemalloc-3.2.2.jar:{}/org/lwjgl/lwjgl-openal/3.2.2/lwjgl-openal-3.2.2.jar:{}/org/lwjgl/lwjgl-opengl/3.2.2/lwjgl-opengl-3.2.2.jar:{}/org/lwjgl/lwjgl-glfw/3.2.2/lwjgl-glfw-3.2.2.jar:{}/org/lwjgl/lwjgl-stb/3.2.2/lwjgl-stb-3.2.2.jar:{}/org/lwjgl/lwjgl-tinyfd/3.2.2/lwjgl-tinyfd-3.2.2.jar:{}/com/mojang/text2speech/1.12.4/text2speech-1.12.4.jar:{}/versions/ForgeOptiFine 1.18.2/ForgeOptiFine 1.18.2.jar", p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, p, game.path_to_minecraft.to_string()));
}

pub fn get_flags(game: &GameManager) -> Vec<String> {
    let mut flags = vec![];

    get_const_vec_flags(&mut flags);
    get_vec_library(game, &mut flags);

    let path_to_minecraft = game.path_to_minecraft.to_string();
    flags.push("--gameDir".to_string());
    flags.push(path_to_minecraft);

    let path_to_assets = SPathBuf(game.path_to_minecraft.0.join("assets")).to_string();
    flags.push("--assertDir".to_string());
    flags.push(path_to_assets);

    flags.push("--username".to_string());
    flags.push(game.username.clone());

    let min_use_memory = format!("-Xms{}M", game.min_use_memory.as_u64() / bytesize::MIB);
    flags.push(min_use_memory);

    let max_use_memory = format!("-Xmx{}M", game.max_use_memory.as_u64() / bytesize::MIB);
    flags.push(max_use_memory);

    flags    
}
