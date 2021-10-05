import React from 'react';
import { Grid } from 'semantic-ui-react';
import "./Status.css";

export class Status extends React.Component {
    render() {
        return (
            <>
                <Grid columns='equal' padded="horizontally" className="status">
                    <Grid.Row>
                        <Grid.Column>Connected:</Grid.Column>
                        <Grid.Column>No</Grid.Column>
                    </Grid.Row>
                    <Grid.Row>
                        <Grid.Column>Sent:</Grid.Column>
                        <Grid.Column>0 B</Grid.Column>
                    </Grid.Row>
                    <Grid.Row>
                        <Grid.Column>Received:</Grid.Column>
                        <Grid.Column>0 B</Grid.Column>
                    </Grid.Row>
                    <Grid.Row>
                        <Grid.Column>Speed:</Grid.Column>
                        <Grid.Column>0 Bps</Grid.Column>
                    </Grid.Row>
                </Grid>
            </>
        );
    }
}
