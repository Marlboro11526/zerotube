import React, { Component } from 'react';
import ReactDOM from 'react-dom';
import './index.css';
import * as serviceWorker from './serviceWorker';
import LoginResponse from './messages/login';
import AuthComponent from './auth/auth';
import TimeComponent from './time/time';
import validator from 'validator';

interface AppProperties {

}

interface AppState {
    error?: string,
    isLoggedIn?: boolean,
    registrationConfirmation?: string,
    serverTime?: Date,
    text?: string,
    user?: string,
}

class App extends Component<AppProperties, AppState> {
    constructor(props: AppProperties) {
        super(props);

        let registrationConfirmation = undefined;

        console.log(window.location.pathname);

        if (window.location.pathname.startsWith("/confirm/")) {
            let id = window.location.pathname.slice("/confirm/".length);
            if (validator.isUUID(id)) {
                registrationConfirmation = id;
            }
        }

        this.state = {
            error: undefined,
            registrationConfirmation: registrationConfirmation,
            isLoggedIn: undefined,
            serverTime: undefined,
            text: undefined,
            user: undefined,
        };

        console.log(registrationConfirmation);
        console.log(this.state);
    }

    componentDidMount() {
        if (this.state.registrationConfirmation) {
            fetch('http://localhost:8081/auth/register/' + this.state.registrationConfirmation, {
                method: 'GET',
                credentials: 'include',
            })
                .then(async response => {
                    if (response.ok) {
                        return;
                    } else {
                        if (response === null) {
                            this.setState({
                                error: "Null response"
                            });
                        } else if (response.body === null) {
                            this.setState({
                                error: "Null body"
                            });
                        } else {
                            let error = await response.json();

                            this.setState({
                                error: error
                            });
                        }
                    }
                });
        }

        fetch('http://localhost:8081/', {
            method: 'GET',
            credentials: 'include',
        })
            .then(response => response.json() as Promise<LoginResponse>)
            .then(response => {
                if (response !== null && response.username !== null) {
                    console.log('RESPONSE: ');
                    console.log(response);
                    this.setState({
                        isLoggedIn: true,
                        user: response.username,
                    });
                } else {
                    this.setState({
                        isLoggedIn: false
                    });
                }
            })
            .catch((e) => console.log(e));
    }

    showSecret(): void {
        console.log("STATE:");
        console.log(this.state);

        fetch('http://localhost:8081/web/secret', {
            method: 'GET',
            credentials: 'include',
        })
            .then(response => {
                if (response.ok) {
                    response.json().then(json => {
                        this.setState({
                            text: json
                        })
                    });

                    return;
                } else {
                    this.setState({
                        text: 'NOT AUTH\'D!!!'
                    });
                }
            })
            .catch((e) => console.log(e))
    }

    handleAuth(response: LoginResponse): void {
        console.log("HANDLING AUTH");

        if (response && response.username) {
            this.setState({
                isLoggedIn: true,
                user: response.username,
            });
        } else {
            this.setState({
                isLoggedIn: false,
                user: undefined,
            });
        }
    }

    render(): JSX.Element {
        if (window.location.pathname.startsWith("/confirm/")) {
            let id = window.location.pathname.slice("/confirm/".length);
            if (validator.isUUID(id)) {
                return <>
                    <h1>{id}</h1>
                    <h2>{this.state.error}</h2>
                    <p><a href="/">Return to home</a></p>
                </>
            }
        }

        if (this.state === null || this.state.isLoggedIn === undefined) {
            return <span id='loader' />
        }

        let loggedInText;

        if (this.state.isLoggedIn) {
            loggedInText = <p>User: {this.state.user}</p>;
        } else {
            loggedInText = <p>Not logged in...</p>
        }

        const isDebug = true;
        let debugZone;

        if (isDebug) {
            debugZone = <>
                <br />
                <br />
                <p>~DEBUG ZONE~</p>
                <p>Is Logged In: {this.state.isLoggedIn ? 'Yes' : 'No'}</p>
                <p>Text: '{this.state.text}'</p>
                <p>User: {this.state.user}</p>
            </>
        }

        let time;

        if (this.state.isLoggedIn) {
            time = <TimeComponent />
        }

        console.log('END OF RENDER:');
        console.log(this.state);

        return (
            <div>
                <AuthComponent
                    authHandler={(response: LoginResponse) => this.handleAuth(response)}
                    isLoggedIn={this.state.isLoggedIn}
                />

                <button onClick={() => this.showSecret()}>Secret</button>
                {loggedInText}
                <p>{this.state.text}</p>
                {debugZone}
                {time}
            </div>
        )
    }
}

ReactDOM.render(<App />, document.getElementById('root'));

// If you want your app to work offline and load faster, you can change
// unregister() to register() below. Note this comes with some pitfalls.
// Learn more about service workers: https://bit.ly/CRA-PWA
serviceWorker.unregister();
