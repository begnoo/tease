import { ChakraProvider } from '@chakra-ui/react'
import { QueryClient, QueryClientProvider } from 'react-query';
import { BrowserRouter } from 'react-router-dom';
import Router from './components/Router';
import SiteLayout from './components/SiteLayout';
import { AuthProvider } from './providers/AuthProvider';

const queryClient = new QueryClient(
  {
    defaultOptions: {
      queries: {
        retry: true,
        cacheTime: 0
      }
    }
  }
);

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <AuthProvider>
        {/* <WithClient> */}
        <BrowserRouter>
          <ChakraProvider>
            <SiteLayout>
              <Router />
            </SiteLayout>
          </ChakraProvider>
          {/* </WithClient> */}
        </BrowserRouter>
      </AuthProvider>
    </QueryClientProvider>
  );
}

export default App;
