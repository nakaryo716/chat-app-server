# FROM nginx:alpine

# RUN rm -f /etc/nginx/conf.d/*

# ADD nginx.conf /etc/nginx/conf.d/portfolio.conf

# CMD /usr/sbin/nginx -g 'daemon off;'
FROM nginx
COPY ./ssl/server.crt /etc/nginx/ssl/server.crt
COPY ./ssl/server.key /etc/nginx/ssl/server.key
COPY ./ssl/server.password /etc/nginx/ssl/server.password
COPY ssl.conf /etc/nginx/conf.d/ssl.conf
COPY nginx.conf /etc/nginx/conf.d/portfolio.conf
