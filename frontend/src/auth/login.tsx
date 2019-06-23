import React, { Component, ChangeEvent } from 'react';
import AuthResponse from '../messages/auth';

interface LoginFormProperties {
    isLoggedIn?: boolean;
    authHandler: any;
}

interface LoginFormState {
    password: string,
    username: string,
}

class LoginForm extends Component<LoginFormProperties, LoginFormState> {
    constructor(props: LoginFormProperties) {
        super(props);
        this.state = {
            username: '',
            password: '',
        };

        this.handleInputChange = this.handleInputChange.bind(this);
    }

    handleInputChange(event: ChangeEvent<HTMLInputElement>) {
        const target = event.target!;
        const name = target.name;
        const value = target.type === 'checkbox' ? target.checked : target.value;

        this.setState({
            [name]: value,
        } as Pick<LoginFormState, any>);
    }

    login(): void {
        fetch('http://localhost:8081/auth/login', {
            method: 'POST',
            credentials: 'include',
            body: JSON.stringify(this.state)
        })
            .then(response => response.json() as Promise<AuthResponse>)
            .then(response => {
                console.log(response);
                this.props.authHandler(response)
            })
            .catch((e) => {
                console.log(e)
            })
    }

    logout(): void {
        fetch('http://localhost:8081/auth/logout', {
            method: 'POST',
            credentials: 'include',
        })
            .then(response => response.json() as Promise<AuthResponse>)
            .then(response => {
                console.log(response);
                this.props.authHandler(response);
            })
            .catch((e) => {
                console.log(e)
            })
    }

    render() {
        let loginForm;
        let loginButton;
        let logoutButton;

        if (this.props.isLoggedIn) {
            logoutButton = <button onClick={() => this.logout()}>Logout</button>;
        } else {
            loginButton = <button onClick={() => this.login()}>Login</button>;

            loginForm = <>
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
            </>
        }

        return (
            <form>
                {loginForm}
                {loginButton}
                {logoutButton}
            </form>
        )
    }
}

export default LoginForm;
