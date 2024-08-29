import { ApolloClient, InMemoryCache } from '@apollo/client';

require('dotenv').config()


const BACKEND_URI = process.env.BACKEND_URI || '';

const apolloClient = new ApolloClient({
  uri: BACKEND_URI,
  cache: new InMemoryCache(),
});
export default apolloClient
