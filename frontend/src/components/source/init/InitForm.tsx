import { Button, Flex, FormControl, FormLabel, Input, Switch, Text, Textarea } from "@chakra-ui/react";
import { AxiosError } from "axios";
import { useContext } from "react";
import { FieldValues, useForm } from "react-hook-form";
import { useMutation } from "react-query";
import { useRequestToast } from "../../../hooks/useRequestToast";
import { AuthContext } from "../../../providers/AuthProvider";
import { initSource, InitSourceRequest } from "../../../services/SourceService";


const InitForm = () => {

    const {
        handleSubmit,
        register,
        setValue,
        formState: { isSubmitting },
    } = useForm();
    const { toastSuccess, toastFailure } = useRequestToast("You've successfully initialized a source.", "Couldn't initialize source")
    const { user } = useContext(AuthContext);


    const { mutate: postInitSource } = useMutation(
        initSource,
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
        const source = {
            ...values,
            owner: user?.email
        }
        postInitSource(source as InitSourceRequest);
    }

    return (
        <form onSubmit={handleSubmit(onSubmit)}>
            <FormControl mt="4px">
            <FormLabel fontSize={"14px"} htmlFor='name'>Name</FormLabel>
            <Input
                id='name'
                placeholder='Name'
                {...register('name', {
                required: 'This is required',
                })}
            />
            </FormControl>

            <FormControl mt="4px">
                <FormLabel fontSize={"14px"} htmlFor='description'>Description</FormLabel>
                <Textarea
                    id='description'
                    placeholder='Description'
                    {...register('description')}
                />
            </FormControl>
            <FormControl mt="4px" justifyContent={"space-between"} alignItems={"center"}>
                <Flex gap={"10px"} mt={"10px"}>
                    <Text>
                        Public
                    </Text>
                    <Switch
                        {...register("visability")} 
                        onChange={(e) => {
                            setValue("visability", e.target.value);
                        }}
                    />
                    <Text>
                        Private
                    </Text>
                </Flex>
            </FormControl>


            <Flex justifyContent="right">
                <Button 
                    mt={5} 
                    colorScheme='teal' 
                    variant="outline"  
                    isLoading={isSubmitting} 
                    type='submit'>
                    Init Source
                </Button>
            </Flex>

        </form>
    ); 
};

export default InitForm;