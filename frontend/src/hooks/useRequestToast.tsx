import { useToast } from "@chakra-ui/react"
import { AxiosError } from "axios";
import { BackendError } from "../services/apiClient";

export const useRequestToast = (successMessage: string, failureMessage: string) => {
    const toast = useToast();

    const toastSuccess = () => {
        toast({
            title: 'Success.',
            description: successMessage,
            status: 'success',
            duration: 1000,
            position: "top",
            isClosable: true,
        });
    }

    const toastFailure = (err: AxiosError) => {
        const backendError = err.response?.data as BackendError;
        toast({
            title: 'Failure.',
            description: `${failureMessage}: ${backendError.error}`,
            status: 'error',
            duration: 2000,
            position: "top",
            isClosable: true,
        });
    }

    return {
        toastSuccess, toastFailure
    }
}