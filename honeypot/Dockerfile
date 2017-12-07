FROM httpd:alpine
RUN sed -i 's|#LoadModule include_module|LoadModule include_module|' conf/httpd.conf
RUN sed -i 's|Options Indexes FollowSymLinks|Options Indexes FollowSymLinks Includes\nAddType text/html .html\nAddOutputFilter INCLUDES .html\n|' conf/httpd.conf
ADD ./html/ /usr/local/apache2/htdocs/
