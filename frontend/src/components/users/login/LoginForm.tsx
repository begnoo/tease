import { useToast } from "@chakra-ui/toast";
import { useContext } from "react";
import { useNavigate } from "react-router";
import { AuthContext } from "../../../providers/AuthProvider";
import { useForm } from "react-hook-form";
import { FormControl, FormErrorMessage, FormLabel } from "@chakra-ui/form-control";
import { Input } from "@chakra-ui/input";
import { Flex } from "@chakra-ui/layout";
import { Button } from "@chakra-ui/button";
import { useMutation } from "react-query";
import { addUserIntoLocalStorage } from "../../../utils/jwtUtils";
import { EMAIL_PATTERN } from "../../../utils/validation";
import { loginUser } from "../../../services/AuthService";
import { AxiosError } from "axios";

export default function LoginForm() {

    const {
        handleSubmit,
        register,
        formState: { errors, isSubmitting },
    } = useForm();

    const navigate = useNavigate();
    const toast = useToast();
    const { login } = useContext(AuthContext);

    const { mutate: postLoginInfo } = useMutation(
        loginUser,
        {
            onSuccess: (res) => {
                console.log(res);
                toast({
                    title: 'Success.',
                    description: `You've successfully logged in.`,
                    status: 'success',
                    duration: 1000,
                    position: "top",
                    isClosable: true,
                });
                addUserIntoLocalStorage(res.data);
                login();
                setTimeout(() => navigate("/"), 500);
            },
            onError: (err: AxiosError) => {
                toast({
                    title: 'Failure.',
                    description: `Couldn't login: ${err.message}`,
                    status: 'error',
                    duration: 1000,
                    position: "top",
                    isClosable: true,
                });
            }
        }
    );

    const onSubmit = (values: any) => {
        postLoginInfo(values);
    }

    return (
        <form onSubmit={handleSubmit(onSubmit)}>
            <FormControl mt="4px">
                <FormLabel fontSize={"14px"} htmlFor='email'>Email</FormLabel>
                <Input
                    id='email'
                    placeholder='Email'
                    {...register('email', {
                        required: 'This is required',
                        validate: {
                            isEmail: (value) => EMAIL_PATTERN.test(value) || "Please enter a valid email address."
                        }
                    })}
                />
            </FormControl>

            <FormControl mt="4px">
                <FormLabel fontSize={"14px"} htmlFor='password'>Password</FormLabel>
                <Input
                    id='password'
                    placeholder='Password'
                    {...register('password', {
                        required: 'This is required',
                    })}
                />
            </FormControl>

            <Flex justifyContent="space-between" alignItems={"center"}>
                <Button
                    mt={5}
                    colorScheme='teal'
                    variant="outline"
                    isLoading={isSubmitting}
                    type='submit'>
                    Log in
                </Button>
            </Flex>

        </form>
    );
}