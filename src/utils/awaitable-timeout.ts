export function awaitableTimeout(duration: number = 1000) {
  return new Promise((resolve, reject) => {
    setTimeout(() => {
      resolve(true);
    }, duration);
  });
}
