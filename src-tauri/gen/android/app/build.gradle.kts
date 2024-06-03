import java.io.FileInputStream
import java.util.Properties

plugins {
	id("com.android.application")
	id("org.jetbrains.kotlin.android")
	id("rust")
}

val tauriProperties = Properties().apply {
	val propFile = file("tauri.properties")
	if (propFile.exists()) {
		propFile.inputStream().use { load(it) }
	}
}

val keystoreProperties = Properties()
val keystorePropertiesFile = rootProject.file("key.properties")

if (keystorePropertiesFile.exists()) {
	keystoreProperties.load(FileInputStream(keystorePropertiesFile))
}

android {
	compileSdk = 34
	namespace = "org.radhaskitchen.mobile"
	defaultConfig {
		manifestPlaceholders["usesCleartextTraffic"] = "false"
		applicationId = "org.radhaskitchen.mobile"
		minSdk = 24
		targetSdk = 34
		versionCode = tauriProperties.getProperty("tauri.android.versionCode", "1").toInt()
		versionName = tauriProperties.getProperty("tauri.android.versionName", "1.0")
	}
	signingConfigs {
		create("release") {
			keyAlias = keystoreProperties.getProperty("keyAlias");
			keyPassword = keystoreProperties.getProperty("keyPassword");
			storeFile = file(keystoreProperties.getProperty("storeFile"));
			storePassword = keystoreProperties.getProperty("storePassword");
		}
	}
	buildTypes {
		getByName("debug") {
			manifestPlaceholders["usesCleartextTraffic"] = "true"
			isDebuggable = true
			isJniDebuggable = true
			isMinifyEnabled = false
			signingConfig = signingConfigs.getByName("debug")
			packaging {
				jniLibs.keepDebugSymbols.add("*/arm64-v8a/*.so")
				jniLibs.keepDebugSymbols.add("*/armeabi-v7a/*.so")
				jniLibs.keepDebugSymbols.add("*/x86/*.so")
				jniLibs.keepDebugSymbols.add("*/x86_64/*.so")
			}
		}
		getByName("release") {
			isMinifyEnabled = true
			signingConfig = signingConfigs.getByName("release")
			proguardFiles(
				*fileTree(".") { include("**/*.pro") }
					.plus(getDefaultProguardFile("proguard-android-optimize.txt"))
					.toList().toTypedArray()
			)
		}
	}
	kotlinOptions {
		jvmTarget = "1.8"
	}
}

rust {
	rootDirRel = "../../../"
}

dependencies {
	implementation("androidx.webkit:webkit:1.6.1")
	implementation("androidx.appcompat:appcompat:1.6.1")
	implementation("com.google.android.material:material:1.8.0")
	testImplementation("junit:junit:4.13.2")
	androidTestImplementation("androidx.test.ext:junit:1.1.4")
	androidTestImplementation("androidx.test.espresso:espresso-core:3.5.0")
}

apply(from = "tauri.build.gradle.kts")