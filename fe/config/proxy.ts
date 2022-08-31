const proxys = {
  testServer: 'http://192.168.0.14:9520',
  local: 'http://127.0.0.1:5000',
  lsLocal: 'http://127.0.0.1:8080/',
};

export default {
  '/api': {
    target: proxys.testServer,
    changeOrigin: true,
  },
};
