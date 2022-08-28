import { Button, Flex, FormControl, FormLabel, Input } from "@chakra-ui/react";
import { AxiosError } from "axios";
import { FieldValues, useForm } from "react-hook-form";
import { useMutation } from "react-query";
import { useRequestToast } from "../../../hooks/useRequestToast";
import { registerUser, RegisterUserRequest } from "../../../services/UserService";
import { EMAIL_PATTERN } from "../../../utils/validation";

const RegisterForm = () => {

    const {
        handleSubmit,
        register,
        getValues,
        formState: { isSubmitting },
    } = useForm();
    const { toastSuccess, toastFailure } = useRequestToast("You've successfully registerd.", "Couldn't register");


    const { mutate: postRegisterUser } = useMutation(
        registerUser,
        {
            onSuccess: (res) => {
                toastSuccess();
            },
            onError: (err: AxiosError) => {
                toastFailure(err);
            }
        }
    );

    const onSubmit = (values: FieldValues) => {
        postRegisterUser(values as RegisterUserRequest);
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
                type="password"
                {...register('password', {
                required: 'This is required',
                validate: {
                    passwordRules: (password) => password.length >= 8 || "Password must contain at least 8 chars long.",           }
                })}
            />
            </FormControl>

            <FormControl mt="4px">
            <FormLabel fontSize={"14px"} htmlFor='password-confirm'>Repeat Password</FormLabel>
            <Input
                id='password-confirm'
                placeholder='Repeat password'
                type="password"
                {...register('password-confirm', {
                required: 'This is required',
                validate: {
                    sameAsPassword: (passwordConfirmation) => passwordConfirmation === getValues()["password"] || "Passwords are not matching"
                }
                })}
            />
            </FormControl>

            <Flex gap={"10px"}>
                <FormControl mt="4px">
                    <FormLabel fontSize={"14px"} htmlFor='firstName'>First name</FormLabel>
                    <Input
                        id='firstName'
                        placeholder='First name'
                        {...register('profile.firstName', {
                        required: 'This is required',
                        })}
                    />
                </FormControl>
                <FormControl mt="4px">
                    <FormLabel fontSize={"14px"} htmlFor='lastName'>Last name</FormLabel>
                    <Input
                        id='lastName'
                        placeholder='Last name'
                        {...register('profile.lastName', {
                        required: 'This is required',
                        })}
                    />
                </FormControl>
            </Flex>

            <Flex justifyContent="right">
                <Button 
                    mt={5} 
                    colorScheme='teal' 
                    variant="outline"  
                    isLoading={isSubmitting} 
                    type='submit'>
                    Register
                </Button>
            </Flex>

        </form>
    ); 
};

export default RegisterForm;