### 介绍

> 本项目用来仿真和测试数据从硬件或其它网络源经三种前端导入rabbimq队列后读取处理的过程。这三种前端是：HTTP、TCP、和UDP。

### 准备

> 需要安装以下三个软件，简单测试可安装在同一台电脑上。测试过程需要打开多个终端窗口，需要把nodejs执行文件安装在搜索路径下。ab.exe在httpd的安装目录的bin子目录下。Rabbitmq做缺省安装，web管理帐号是guest:guest，不做改动。

* Apache httpd
  - 软件包里的ab.exe用来做压力测试
  - http://archive.apache.org/dist/httpd/binaries/win32/httpd-2.2.25-win32-x86-openssl-0.9.8y.msi
* Nodejs
  - javascript 服务器端开发平台，功能强大
  - https://nodejs.org
* Rabbitmq
  - http://www.rabbitmq.com/

### 文件介绍

> 三种前端中，HTTP用ab.exe导入数据，脚本在ab-get.bat中。其它两种是js文件，用node.exe命令启动。

* 数据导入程序和脚本
  - HTTP: ab-get.bat
  - TCP:  socket-io-client.js
  - UDP:  udp-client.js
* 前端服务器程序
  - HTTP: http-server.js
  - TCP： socket-io-server.js
  - UDP:  udp-server.js
* 消息队列读取程序
  - subscriber.js

### 测试步骤

1. 把软件解压到目录：rabbitmq-test下（以下作为例子）；
2. 打开三个终端窗口，进入rabbitmq-test目录下；
3. 在第一个窗口中运行npm install，安装必要的nodejs软件包；
4. 在第一个窗口中运行node subscriber.js，启动消息队列读取程序；
5. 在第二个窗口中运行node *-server.js程序，启动前端服务器程序；
6. 在第三个窗口中运行node *-client.js程序（对HTTP，直接运行ab-get.bat），启动数据导入程序。

### 注意事项

1. 不同的前端服务器程序对应不同的数据导入程序。
2. 修改ab-get.bat文件到对应的运行程序目录。
3. HTTP前端服务器程序，可用浏览器打开做测试。
4. 按CTRL+C，终止程序运行。
