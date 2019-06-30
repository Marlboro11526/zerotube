import React, { Component, FormEvent } from 'react';

interface LogoutFormProperties {
    handleClose: any;
    logoutHandler: any;
}

class LogoutForm extends Component<LogoutFormProperties> {
    constructor(props: LogoutFormProperties) {
        super(props);

        this.handleLogoutSubmit = this.handleLogoutSubmit.bind(this);
    }

    async handleLogoutSubmit(event: FormEvent<HTMLFormElement>) {
        console.log("SUBMIT LOGOUT");
        event.preventDefault();

        try {
            await this.props.logoutHandler();
        } catch (error) {
            console.log("CAUGHT");

            return;
        }
    }

    render() {
        let logoutForm = <form onSubmit={this.handleLogoutSubmit}>
            <p>Are you sure?</p>
            <input type="submit" value="Logout" />
            <button type="button" onClick={() => this.props.handleClose()}>Cancel</button>
        </form>

        return (
            <div className="modal-body">
                {logoutForm}
            </div>
        );
    }
}

export default LogoutForm;
