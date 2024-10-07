
export function getEnvironment() {
  const env = process.env;
  const BACKEND_URI = process.env.NODE_ENV === "production"
    ? process.env.BACKEND_URI_PROD ?? ''
    : process.env.BACKEND_URI_DEV ?? ''

  return {
    NODE_ENV: env.NODE_ENV,
    BACKEND_URI: BACKEND_URI,
  }
}



