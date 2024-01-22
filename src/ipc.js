/**
 * IPC communication function
 * @param method The method name to call
 * @param args The arguments to pass to the remote function
 * @param callback Whether a callback is needed
 */
function ipc_send(method, args, callback) {
  const payload = {
    method: method,
    args: args,
  };
  let promise = null;

  if (callback) {
    const name = window.crypto.randomUUID().replaceAll("-", "");
    promise = new Promise((resolve) => {
      window[name] = function (response) {
        resolve(response);
        delete window[name];
      };
    });
    payload.callback = name;
  }

  window.ipc.postMessage(JSON.stringify(payload));
  return promise;
}

/**
 * Install the WRY ipc plugin
 * @param app vue 3 app instance
 */
function plugin(app) {
  if (!window.ipc) {
    // for debug purposes
    console.warn("IPC bridge is not available.");
    window.ipc = {
      postMessage: function (message) {
        console.info(`[ipc] ${message}`);
      },
    };
  }
  // IPC module shim
  const ipc = {
    call: function (method, args) {
      return ipc_send(method, args, true);
    },
    notify: function (signal, args) {
      ipc_send(signal, args);
    },
  };
  app.config.globalProperties["$ipc"] = ipc;
  app["$ipc"] = ipc;
}

export default plugin;
