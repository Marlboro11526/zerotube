import React, { Component } from "react";
import LoginResponse from "../messages/login";
import ErrorResponse from "../messages/error";
import LoginForm from "./login";
import RegisterForm from "./register";
import RegisterResponse from "../messages/register";
import LogoutForm from "./logout";

interface AuthProperties {
    isLoggedIn?: boolean;
    authHandler: any;
}

interface AuthState {
    showLoginForm: boolean;
    showLogoutConfirmation: boolean;
    showRegisterForm: boolean;
}

class AuthComponent extends Component<AuthProperties, AuthState> {
    constructor(props: AuthProperties) {
        super(props);

        this.state = {
            showLoginForm: false,
            showLogoutConfirmation: false,
            showRegisterForm: false,
        };
    }

    showLoginForm(isActive: boolean): void {
        this.setState({
            showLoginForm: isActive
        });
    }

    showLogoutConfirmation(isActive: boolean): void {
        this.setState({
            showLogoutConfirmation: isActive
        });
    }

    showRegisterForm(isActive: boolean): void {
        this.setState({
            showRegisterForm: isActive
        });
    }

    async handleLogin(username: string, password: string): Promise<void> {
        let response;

        try {
            response = await this.login(username, password);
        } catch (error) {
            console.log("CAUGHT " + error);

            throw error;
        }

        console.log("SETTING RESPONSE: ");
        console.log(response);

        this.props.authHandler(response);

        this.setState({
            showLoginForm: false
        });
    }

    async handleLogout(): Promise<void> {
        let response;

        try {
            response = await this.logout();
        } catch (error) {
            console.log("CAUGHT " + error);

            throw error;
        }

        console.log("SETTING RESPONSE: ");
        console.log(response);

        this.props.authHandler(response);

        this.setState({
            showLogoutConfirmation: false
        });
    }

    async handleRegister(email: string, username: string, password: string): Promise<void> {
        let response;

        try {
            response = await this.register(email, username, password);
        } catch (error) {
            console.log("CAUGHT " + error);

            throw error;
        }

        console.log("SETTING RESPONSE: ");
        console.log(response);

        this.props.authHandler(response);

        this.setState({
            showRegisterForm: false
        });
    }

    async login(username: string, password: string): Promise<LoginResponse> {
        return fetch('http://localhost:8081/auth/login', {
            method: 'POST',
            credentials: 'include',
            headers: new Headers([['Content-Type', "application/json"]]),
            body: JSON.stringify({ username, password })
        })
            .then(async response => {
                let json = await response.json();

                if (response.ok) {
                    return json as LoginResponse;
                } else {
                    let error = json as ErrorResponse;

                    if (error !== null) {
                        throw new Error(error.error);
                    } else {
                        let error = await response.text();

                        throw new Error("Error in login: " + response.status + "\n" + error);
                    }
                }
            });
    }

    async logout(): Promise<LoginResponse> {
        return fetch('http://localhost:8081/auth/logout', {
            method: 'POST',
            credentials: 'include',
        })
            .then(async response => {
                if (response.ok) {
                    return response.json() as Promise<LoginResponse>;
                } else {
                    if (response === null) {
                        throw new Error("null response");
                    } else if (response.body === null) {
                        throw new Error("null body");
                    } else {
                        let error = await response.text();

                        throw new Error(error);
                    }
                }
            });
    }

    async register(email: string, username: string, password: string): Promise<RegisterResponse> {
        return fetch('http://localhost:8081/auth/register', {
            method: 'POST',
            credentials: 'include',
            headers: new Headers([['Content-Type', "application/json"]]),
            body: JSON.stringify({ email, username, password })
        })
            .then(async response => {
                if (response.ok) {
                    console.log(response.text());

                    return response.json() as Promise<RegisterResponse>;
                } else {
                    if (response === null) {
                        throw new Error("null response");
                    } else if (response.body === null) {
                        throw new Error("null body");
                    } else {
                        let error = await response.text();

                        throw new Error(error);
                    }
                }
            });
    }

    render() {
        let authForm;
        let buttons;

        if (this.state.showLoginForm) {
            authForm = <div className="modal">
                <LoginForm
                    loginHandler={(username: string, password: string) => this.handleLogin(username, password)}
                    handleClose={() => this.showLoginForm(false)}
                />
            </div>
        } else if (this.state.showRegisterForm) {
            authForm = <div className="modal">
                <RegisterForm
                    registerHandler={(email: string, username: string, password: string) => this.handleRegister(email, username, password)}
                    handleClose={() => this.showRegisterForm(false)}
                />
            </div>
        } else if (this.state.showLogoutConfirmation) {
            authForm = <div className="modal">
                <LogoutForm
                    logoutHandler={() => this.handleLogout()}
                    handleClose={() => this.showLogoutConfirmation(false)}
                />
            </div>
        }

        if (this.props.isLoggedIn) {
            buttons = <button type="button" onClick={() => this.showLogoutConfirmation(true)}>Logout</button>
        } else {
            buttons = <>
                <button type="button" onClick={() => this.showLoginForm(true)}>Login</button>
                <button type="button" onClick={() => this.showRegisterForm(true)}>Register</button>
            </>
        }

        return (
            <div>
                {authForm}
                {buttons}
            </div>
        );
    }
}

export default AuthComponent;
