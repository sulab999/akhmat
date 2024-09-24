# arkhmat  
## 简介
基于rust的环境检测SDK  
致力于跨平台安全SDK研究  

## 主要功能
- [√] root检测
- [√] 模拟器检测
- [√] frida检测
- [√] 兼容鸿蒙
- [×] 调试检测
- [×] 兼容ios

## 效果截图
demo app见releases  
![sdkdemo](https://github.com/sulab999/akhmat/blob/master/img/sdkdemo.jpg)
## 适用场景
- 环境检测
- 风控检测
- 移动安全测试

## 环境说明
开发语言：rust  
编译后的so文件兼容的环境：安卓、鸿蒙

## sdk调用开发
安卓  
1、新建Nativegolib类  
2、加载akhmat.so  
3、调用native方法  
调用示例  
![sdkdiaoyong](https://github.com/sulab999/akhmat/blob/master/img/sdkdiaoyong.jpg)

# 公众号
水平有限目前比较low，欢迎rust开发或移动安全的朋友们来交流  
加群：关注公众号并回复sdk
![webchat](https://github.com/sulab999/Taichi/blob/main/webchat.png)  
# 免责声明
请勿将本项目技术或代码应用在恶意软件制作、软件著作权/知识产权盗取或不当牟利等非法用途中。实施上述行为或利用本项目对非自己著作权所有的程序进行数据嗅探将涉嫌违反《中华人民共和国刑法》第二百一十七条、第二百八十六条，《中华人民共和国网络安全法》《中华人民共和国计算机软件保护条例》等法律规定。本项目提及的技术仅可用于私人学习测试等合法场景中，任何不当利用该技术所造成的刑事、民事责任均与本项目作者无关。