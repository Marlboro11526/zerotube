import React, { Component, ChangeEvent, FormEvent } from 'react';

interface RegisterFormProperties {
    handleClose: any;
    registerHandler: any;
}

interface RegisterFormState {
    email?: string,
    error?: string,
    password?: string,
    username?: string,
}

class RegisterForm extends Component<RegisterFormProperties, RegisterFormState> {
    constructor(props: RegisterFormProperties) {
        super(props);

        this.state = {
            email: undefined,
            error: undefined,
            password: undefined,
            username: undefined,
        };

        this.handleInputChange = this.handleInputChange.bind(this);
        this.handleRegisterSubmit = this.handleRegisterSubmit.bind(this);
    }

    handleInputChange(event: ChangeEvent<HTMLInputElement>) {
        const target = event.target!;
        const name = target.name;
        const value = target.type === 'checkbox' ? target.checked : target.value;
        console.log(name + " changed to " + value);

        this.setState({
            [name]: value,
        } as Pick<RegisterFormState, any>);
    }

    async handleRegisterSubmit(event: FormEvent<HTMLFormElement>) {
        console.log("SUBMIT REGISTER");
        event.preventDefault();

        if (this.state.email && this.state.username && this.state.password) {
            try {
                console.log("SENDING " + this.state.email + " " + this.state.username + " " + this.state.password);
                await this.props.registerHandler(this.state.email, this.state.username, this.state.password);
            } catch (error) {
                console.log("CAUGHT: " + error);
                this.setState({
                    error: "Unable to register this email/username."
                });

                return;
            }
        } else {
            this.setState({
                error: "Please fill out the form."
            });
        }
    }

    render() {
        let errorMessage;

        let registerForm = <form onSubmit={this.handleRegisterSubmit}>
            <label>Email: </label>
            <input
                name='email'
                type='text'
                onChange={this.handleInputChange}
            />
            <label>Username:</label>
            <input
                name='username'
                type='text'
                onChange={this.handleInputChange}
            />
            <label>Password:</label>
            <input
                name='password'
                type='password'
                onChange={this.handleInputChange}
            />
            <input type="submit" value="Register" />
            <button type="button" onClick={() => this.props.handleClose()}>Cancel</button>
        </form>


        if (this.state.error) {
            errorMessage = <p>Error: {this.state.error}</p>
        }

        return (
            <div className="modal-body">
                {errorMessage}
                {registerForm}
            </div>
        );
    }
}

export default RegisterForm;
