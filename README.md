本项目不包含邮件相关配置，需要在项目根目录下新增 config 文件。
示例如下：

``````
#不需要引号
#发件人邮箱
from_email = form.example.com
#收件人邮箱
to_email= to.example.com
smtp_server = smtp.example.com
smtp_port = 123
smtp_user = to.example.com
smtp_password = yourpassword
#主题
subject=Test Email
#内容
body= Hello, this is a test email
``````

告警阈值设置为总shm的90%，告警邮件发送后，程序会停止，需要手动再启动

