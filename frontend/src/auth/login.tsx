import React, { Component, ChangeEvent, FormEvent } from 'react';

interface LoginFormProperties {
    handleClose: any;
    loginHandler: any;
}

interface LoginFormState {
    error?: string,
    password?: string,
    username?: string,
}

class LoginForm extends Component<LoginFormProperties, LoginFormState> {
    constructor(props: LoginFormProperties) {
        super(props);

        this.state = {
            error: undefined,
            password: undefined,
            username: undefined,
        };

        this.handleInputChange = this.handleInputChange.bind(this);
        this.handleLoginSubmit = this.handleLoginSubmit.bind(this);
    }

    handleInputChange(event: ChangeEvent<HTMLInputElement>) {
        const target = event.target!;
        const name = target.name;
        const value = target.type === 'checkbox' ? target.checked : target.value;

        this.setState({
            [name]: value,
        } as Pick<LoginFormState, any>);
    }

    async handleLoginSubmit(event: FormEvent<HTMLFormElement>) {
        console.log("SUBMIT LOGIN");
        event.preventDefault();

        if (this.state.username && this.state.password) {
            try {
                await this.props.loginHandler(this.state.username, this.state.password);
            } catch (error) {
                console.log("CAUGHT");
                this.setState({
                    error: "Could not find username/password."
                });

                return;
            }
        } else {
            this.setState({
                error: "Please enter a username and password."
            });
        }
    }

    render() {
        let errorMessage;

        let loginForm = <form onSubmit={this.handleLoginSubmit}>
            <label>Username:</label>
            <input
                name='username'
                type='text'
                onChange={this.handleInputChange} />
            <label>Password:</label>
            <input
                name='password'
                type='password'
                onChange={this.handleInputChange} />
            <input type="submit" value="Login" />
            <button type="button" onClick={() => this.props.handleClose()}>Cancel</button>
        </form>


        if (this.state.error) {
            errorMessage = <p>Error: {this.state.error}</p>
        }

        return (
            <div className="modal-body">
                {errorMessage}
                {loginForm}
            </div>
        );
    }
}

export default LoginForm;
