pipeline {
    agent any

    options {
        timestamps()
        buildDiscarder(logRotator(numToKeepStr: '20'))
        timeout(time: 15, unit: 'MINUTES')
    }

    environment {
        CARGO_TERM_COLOR = 'always'
    }

    stages {
        stage('Checkout') {
            steps {
                checkout scm
            }
        }

        stage('Format check') {
            steps {
                sh 'cargo fmt --check'
            }
        }

        stage('Lint (clippy)') {
            steps {
                // Project dang WIP (nhieu scaffold chua dung toi), khong chan build vi warning
                // dead_code/unused_variables — chi bao cao de biet, khong -D warnings.
                sh 'cargo clippy --all-targets || true'
            }
        }

        stage('Build') {
            steps {
                sh 'cargo build --verbose'
            }
        }

        stage('Test') {
            steps {
                sh 'cargo test --verbose'
            }
        }
    }

    post {
        success {
            echo 'Build + test thanh cong.'
        }
        failure {
            echo 'Build/test that bai — xem log stage nao do fail o tren.'
        }
        always {
            cleanWs()
        }
    }
}
